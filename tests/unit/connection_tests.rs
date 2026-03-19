use iterm2_client::proto;
use std::sync::Arc;
use std::time::Duration;

use crate::common::mock_server::{self, MockServer};

#[tokio::test]
async fn call_assigns_incrementing_ids() {
    let handler: mock_server::Handler = Arc::new(|req| {
        Some(proto::ServerOriginatedMessage {
            id: req.id,
            submessage: Some(
                proto::server_originated_message::Submessage::ListSessionsResponse(
                    proto::ListSessionsResponse {
                        windows: vec![],
                        buried_sessions: vec![],
                    },
                ),
            ),
        })
    });

    let server = MockServer::start(handler).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp1 = conn
        .call(iterm2_client::request::list_sessions())
        .await
        .unwrap();
    assert_eq!(resp1.id, Some(1));

    let resp2 = conn
        .call(iterm2_client::request::list_sessions())
        .await
        .unwrap();
    assert_eq!(resp2.id, Some(2));

    let resp3 = conn
        .call(iterm2_client::request::list_sessions())
        .await
        .unwrap();
    assert_eq!(resp3.id, Some(3));

    server.shutdown().await;
}

#[tokio::test]
async fn concurrent_calls_get_correct_responses() {
    let handler: mock_server::Handler = Arc::new(|req| {
        match &req.submessage {
            Some(proto::client_originated_message::Submessage::ListSessionsRequest(_)) => {
                Some(proto::ServerOriginatedMessage {
                    id: req.id,
                    submessage: Some(
                        proto::server_originated_message::Submessage::ListSessionsResponse(
                            proto::ListSessionsResponse {
                                windows: vec![],
                                buried_sessions: vec![],
                            },
                        ),
                    ),
                })
            }
            Some(proto::client_originated_message::Submessage::SendTextRequest(_)) => {
                Some(proto::ServerOriginatedMessage {
                    id: req.id,
                    submessage: Some(
                        proto::server_originated_message::Submessage::SendTextResponse(
                            proto::SendTextResponse {
                                status: Some(0),
                            },
                        ),
                    ),
                })
            }
            _ => None,
        }
    });

    let server = MockServer::start(handler).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let conn2 = conn.clone();
    let (r1, r2) = tokio::join!(
        conn.call(iterm2_client::request::list_sessions()),
        conn2.call(iterm2_client::request::send_text("s1", "hello")),
    );

    let r1 = r1.unwrap();
    let r2 = r2.unwrap();

    assert!(matches!(
        r1.submessage,
        Some(proto::server_originated_message::Submessage::ListSessionsResponse(_))
    ));
    assert!(matches!(
        r2.submessage,
        Some(proto::server_originated_message::Submessage::SendTextResponse(_))
    ));

    server.shutdown().await;
}

#[tokio::test]
async fn server_error_string_returns_api_error() {
    let handler: mock_server::Handler = Arc::new(|req| {
        Some(proto::ServerOriginatedMessage {
            id: req.id,
            submessage: Some(proto::server_originated_message::Submessage::Error(
                "malformed request".to_string(),
            )),
        })
    });

    let server = MockServer::start(handler).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let err = conn
        .call(iterm2_client::request::list_sessions())
        .await
        .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("malformed request"), "got: {msg}");

    server.shutdown().await;
}

#[tokio::test]
async fn timeout_returns_error() {
    let handler: mock_server::Handler = Arc::new(|_req| None);

    let server = MockServer::start(handler).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let err = conn
        .call_with_timeout(
            iterm2_client::request::list_sessions(),
            Duration::from_millis(100),
        )
        .await
        .unwrap_err();
    assert!(
        err.to_string().contains("timed out"),
        "got: {}",
        err.to_string()
    );

    server.shutdown().await;
}

#[tokio::test]
async fn notification_broadcast() {
    use futures_util::{SinkExt, StreamExt};
    use prost::Message;
    use tokio_tungstenite::tungstenite;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();
        let (mut sink, mut source) = ws_stream.split();

        if let Some(Ok(tungstenite::Message::Binary(data))) = source.next().await {
            let req = proto::ClientOriginatedMessage::decode(data.as_ref()).unwrap();

            // Send response
            let resp = proto::ServerOriginatedMessage {
                id: req.id,
                submessage: Some(
                    proto::server_originated_message::Submessage::ListSessionsResponse(
                        proto::ListSessionsResponse {
                            windows: vec![],
                            buried_sessions: vec![],
                        },
                    ),
                ),
            };
            let mut buf = Vec::new();
            resp.encode(&mut buf).unwrap();
            let _ = SinkExt::<tungstenite::Message>::send(
                &mut sink,
                tungstenite::Message::Binary(buf.into()),
            )
            .await;

            // Send a spontaneous notification
            tokio::time::sleep(Duration::from_millis(50)).await;
            let notif = proto::ServerOriginatedMessage {
                id: None,
                submessage: Some(
                    proto::server_originated_message::Submessage::Notification(
                        proto::Notification {
                            new_session_notification: Some(proto::NewSessionNotification {
                                session_id: Some("notif-session-1".to_string()),
                            }),
                            ..Default::default()
                        },
                    ),
                ),
            };
            let mut buf = Vec::new();
            notif.encode(&mut buf).unwrap();
            let _ = SinkExt::<tungstenite::Message>::send(
                &mut sink,
                tungstenite::Message::Binary(buf.into()),
            )
            .await;
        }
    });

    let url = format!("ws://{}", addr);
    let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
    let (sink, source) = ws_stream.split();
    let conn = iterm2_client::Connection::from_split(sink, source);

    let mut rx = conn.subscribe_notifications();

    let _resp = conn
        .call(iterm2_client::request::list_sessions())
        .await
        .unwrap();

    let notif = tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(notif.new_session_notification.is_some());
    assert_eq!(
        notif
            .new_session_notification
            .unwrap()
            .session_id
            .as_deref(),
        Some("notif-session-1")
    );

    server_task.abort();
}
