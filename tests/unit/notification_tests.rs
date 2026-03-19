use futures_util::StreamExt;
use iterm2_client::notification;
use iterm2_client::proto;
use tokio::sync::broadcast;

fn make_channel() -> broadcast::Sender<proto::Notification> {
    let (tx, _) = broadcast::channel(16);
    tx
}

#[tokio::test]
async fn keystroke_notifications_filters() {
    let tx = make_channel();
    let rx = tx.subscribe();

    let mut stream = Box::pin(notification::keystroke_notifications(rx));

    // Send a non-keystroke notification (should be skipped)
    tx.send(proto::Notification {
        new_session_notification: Some(proto::NewSessionNotification {
            session_id: Some("s1".to_string()),
        }),
        ..Default::default()
    })
    .unwrap();

    // Send a keystroke notification
    tx.send(proto::Notification {
        keystroke_notification: Some(proto::KeystrokeNotification {
            characters: Some("a".to_string()),
            characters_ignoring_modifiers: None,
            modifiers: vec![],
            key_code: Some(0),
            session: Some("s1".to_string()),
            action: None,
        }),
        ..Default::default()
    })
    .unwrap();

    drop(tx);

    let notif = stream.next().await.unwrap();
    assert_eq!(notif.characters.as_deref(), Some("a"));
}

#[tokio::test]
async fn screen_update_notifications_filters() {
    let tx = make_channel();
    let rx = tx.subscribe();

    let mut stream = Box::pin(notification::screen_update_notifications(rx));

    tx.send(proto::Notification {
        screen_update_notification: Some(proto::ScreenUpdateNotification {
            session: Some("s1".to_string()),
        }),
        ..Default::default()
    })
    .unwrap();
    drop(tx);

    let notif = stream.next().await.unwrap();
    assert_eq!(notif.session.as_deref(), Some("s1"));
}

#[tokio::test]
async fn prompt_notifications_filters() {
    let tx = make_channel();
    let rx = tx.subscribe();

    let mut stream = Box::pin(notification::prompt_notifications(rx));

    tx.send(proto::Notification {
        prompt_notification: Some(proto::PromptNotification {
            session: Some("s1".to_string()),
            event: None,
            unique_prompt_id: Some("p1".to_string()),
        }),
        ..Default::default()
    })
    .unwrap();
    drop(tx);

    let notif = stream.next().await.unwrap();
    assert_eq!(notif.session.as_deref(), Some("s1"));
    assert_eq!(notif.unique_prompt_id.as_deref(), Some("p1"));
}

#[tokio::test]
async fn new_session_notifications_filters() {
    let tx = make_channel();
    let rx = tx.subscribe();

    let mut stream = Box::pin(notification::new_session_notifications(rx));

    // Send irrelevant notification first
    tx.send(proto::Notification::default()).unwrap();

    tx.send(proto::Notification {
        new_session_notification: Some(proto::NewSessionNotification {
            session_id: Some("new-s1".to_string()),
        }),
        ..Default::default()
    })
    .unwrap();
    drop(tx);

    let notif = stream.next().await.unwrap();
    assert_eq!(notif.session_id.as_deref(), Some("new-s1"));
}

#[tokio::test]
async fn terminate_session_notifications_filters() {
    let tx = make_channel();
    let rx = tx.subscribe();

    let mut stream = Box::pin(notification::terminate_session_notifications(rx));

    tx.send(proto::Notification {
        terminate_session_notification: Some(proto::TerminateSessionNotification {
            session_id: Some("term-s1".to_string()),
        }),
        ..Default::default()
    })
    .unwrap();
    drop(tx);

    let notif = stream.next().await.unwrap();
    assert_eq!(notif.session_id.as_deref(), Some("term-s1"));
}

#[tokio::test]
async fn focus_changed_notifications_filters() {
    let tx = make_channel();
    let rx = tx.subscribe();

    let mut stream = Box::pin(notification::focus_changed_notifications(rx));

    tx.send(proto::Notification {
        focus_changed_notification: Some(proto::FocusChangedNotification {
            event: Some(proto::focus_changed_notification::Event::ApplicationActive(
                true,
            )),
        }),
        ..Default::default()
    })
    .unwrap();
    drop(tx);

    let notif = stream.next().await.unwrap();
    assert!(matches!(
        notif.event,
        Some(proto::focus_changed_notification::Event::ApplicationActive(true))
    ));
}

#[tokio::test]
async fn layout_changed_notifications_filters() {
    let tx = make_channel();
    let rx = tx.subscribe();

    let mut stream = Box::pin(notification::layout_changed_notifications(rx));

    tx.send(proto::Notification {
        layout_changed_notification: Some(proto::LayoutChangedNotification {
            list_sessions_response: None,
        }),
        ..Default::default()
    })
    .unwrap();
    drop(tx);

    let notif = stream.next().await.unwrap();
    assert!(notif.list_sessions_response.is_none());
}

#[tokio::test]
async fn variable_changed_notifications_filters() {
    let tx = make_channel();
    let rx = tx.subscribe();

    let mut stream = Box::pin(notification::variable_changed_notifications(rx));

    tx.send(proto::Notification {
        variable_changed_notification: Some(proto::VariableChangedNotification {
            scope: Some(proto::VariableScope::Session as i32),
            identifier: Some("s1".to_string()),
            name: Some("user.foo".to_string()),
            json_new_value: Some(r#""bar""#.to_string()),
        }),
        ..Default::default()
    })
    .unwrap();
    drop(tx);

    let notif = stream.next().await.unwrap();
    assert_eq!(notif.name.as_deref(), Some("user.foo"));
    assert_eq!(notif.json_new_value.as_deref(), Some(r#""bar""#));
}

#[tokio::test]
async fn custom_escape_sequence_notifications_filters() {
    let tx = make_channel();
    let rx = tx.subscribe();

    let mut stream = Box::pin(notification::custom_escape_sequence_notifications(rx));

    tx.send(proto::Notification {
        custom_escape_sequence_notification: Some(proto::CustomEscapeSequenceNotification {
            session: Some("s1".to_string()),
            sender_identity: Some("test-app".to_string()),
            payload: Some("custom-data".to_string()),
        }),
        ..Default::default()
    })
    .unwrap();
    drop(tx);

    let notif = stream.next().await.unwrap();
    assert_eq!(notif.session.as_deref(), Some("s1"));
    assert_eq!(notif.payload.as_deref(), Some("custom-data"));
}

#[tokio::test]
async fn notification_stream_ends_on_channel_close() {
    let tx = make_channel();
    let rx = tx.subscribe();

    let mut stream = Box::pin(notification::new_session_notifications(rx));

    drop(tx);

    let result = stream.next().await;
    assert!(result.is_none());
}

#[tokio::test]
async fn notification_stream_new() {
    use iterm2_client::notification::NotificationStream;

    let tx = make_channel();
    let rx = tx.subscribe();
    let mut stream = NotificationStream::new(rx);

    tx.send(proto::Notification {
        new_session_notification: Some(proto::NewSessionNotification {
            session_id: Some("s1".to_string()),
        }),
        ..Default::default()
    })
    .unwrap();
    drop(tx);

    let notif = stream.next().await.unwrap();
    assert!(notif.new_session_notification.is_some());
}
