use iterm2_client::proto;
use std::sync::Arc;

use crate::common::mock_server::{self, MockServer};

#[tokio::test]
async fn app_list_sessions_parses_tree() {
    let server = MockServer::start(mock_server::list_sessions_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;
    let app = iterm2_client::App::new(conn);

    let result = app.list_sessions().await.unwrap();
    assert_eq!(result.windows.len(), 1);
    assert_eq!(result.windows[0].window.id, "window-1");
    assert_eq!(result.windows[0].tabs.len(), 1);
    assert_eq!(result.windows[0].tabs[0].tab.id, "tab-1");
    assert_eq!(result.windows[0].tabs[0].sessions.len(), 1);
    assert_eq!(result.windows[0].tabs[0].sessions[0].id, "session-1");
    assert_eq!(
        result.windows[0].tabs[0].sessions[0].title.as_deref(),
        Some("bash")
    );

    server.shutdown().await;
}

#[tokio::test]
async fn session_send_text_ok() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = Arc::new(mock_server::connect_to_mock(server.addr).await);
    let session = iterm2_client::Session::new("s1".to_string(), None, conn);

    session.send_text("hello\n").await.unwrap();

    server.shutdown().await;
}

#[tokio::test]
async fn session_get_screen_contents() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = Arc::new(mock_server::connect_to_mock(server.addr).await);
    let session = iterm2_client::Session::new("s1".to_string(), None, conn);

    let lines = session.get_screen_contents().await.unwrap();
    assert_eq!(lines, vec!["hello world"]);

    server.shutdown().await;
}

#[tokio::test]
async fn session_split() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = Arc::new(mock_server::connect_to_mock(server.addr).await);
    let session = iterm2_client::Session::new("s1".to_string(), None, conn);

    let new_ids = session
        .split(
            proto::split_pane_request::SplitDirection::Vertical,
            false,
            None,
        )
        .await
        .unwrap();
    assert_eq!(new_ids, vec!["split-session-1"]);

    server.shutdown().await;
}

#[tokio::test]
async fn session_get_variable() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = Arc::new(mock_server::connect_to_mock(server.addr).await);
    let session = iterm2_client::Session::new("s1".to_string(), None, conn);

    let val = session.get_variable("user.test").await.unwrap();
    assert_eq!(val.as_deref(), Some(r#""test_value""#));

    server.shutdown().await;
}

#[tokio::test]
async fn session_get_profile_property() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = Arc::new(mock_server::connect_to_mock(server.addr).await);
    let session = iterm2_client::Session::new("s1".to_string(), None, conn);

    let props = session
        .get_profile_property(vec!["Name".to_string()])
        .await
        .unwrap();
    assert_eq!(props.len(), 1);
    assert_eq!(props[0].key.as_deref(), Some("Name"));

    server.shutdown().await;
}

#[tokio::test]
async fn session_close() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = Arc::new(mock_server::connect_to_mock(server.addr).await);
    let session = iterm2_client::Session::new("s1".to_string(), None, conn);

    session.close(true).await.unwrap();

    server.shutdown().await;
}

#[tokio::test]
async fn app_create_tab() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;
    let app = iterm2_client::App::new(conn);

    let result = app.create_tab(None, None).await.unwrap();
    assert_eq!(result.session.id, "new-session-1");
    assert_eq!(result.tab.id, "42");
    assert_eq!(result.window.id, "window-1");

    server.shutdown().await;
}

#[tokio::test]
async fn app_focus() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;
    let app = iterm2_client::App::new(conn);

    let notifications = app.focus().await.unwrap();
    assert!(notifications.is_empty());

    server.shutdown().await;
}

#[tokio::test]
async fn app_begin_end_transaction() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;
    let app = iterm2_client::App::new(conn);

    app.begin_transaction().await.unwrap();
    app.end_transaction().await.unwrap();

    server.shutdown().await;
}

#[tokio::test]
async fn window_get_set_property() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = Arc::new(mock_server::connect_to_mock(server.addr).await);
    let window = iterm2_client::Window::new("w1".to_string(), conn);

    let val = window.get_property("frame").await.unwrap();
    assert!(val.is_some());

    window.set_property("fullscreen", "true").await.unwrap();

    server.shutdown().await;
}

#[tokio::test]
async fn non_ok_status_returns_error() {
    let handler: mock_server::Handler = Arc::new(|req| {
        Some(proto::ServerOriginatedMessage {
            id: req.id,
            submessage: Some(
                proto::server_originated_message::Submessage::SendTextResponse(
                    proto::SendTextResponse {
                        status: Some(proto::send_text_response::Status::SessionNotFound as i32),
                    },
                ),
            ),
        })
    });

    let server = MockServer::start(handler).await;
    let conn = Arc::new(mock_server::connect_to_mock(server.addr).await);
    let session = iterm2_client::Session::new("bad-session".to_string(), None, conn);

    let err = session.send_text("hello").await.unwrap_err();
    assert!(err.to_string().contains("status"), "got: {err}");

    server.shutdown().await;
}
