use iterm2_client::proto;
use iterm2_client::proto::client_originated_message::Submessage;
use iterm2_client::request;

#[path = "../common/mod.rs"]
mod common;
use common::mock_server::{self, MockServer};

// Test the request builders that make actual calls through the mock server,
// exercising the full request→response path for all 34 operations.

#[tokio::test]
async fn invoke_function_app_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::invoke_function_app("test()"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::InvokeFunctionResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn invoke_function_session_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::invoke_function_session("s1", "test()"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::InvokeFunctionResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn invoke_function_tab_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::invoke_function_tab("t1", "test()"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::InvokeFunctionResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn invoke_function_window_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::invoke_function_window("w1", "test()"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::InvokeFunctionResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn rpc_result_value_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::rpc_result_value("req-1", r#""ok""#))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::ServerOriginatedRpcResultResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn rpc_result_exception_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::rpc_result_exception(
            "req-1",
            r#"{"reason": "failed"}"#,
        ))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::ServerOriginatedRpcResultResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn register_tool_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::register_tool(
            "My Tool",
            "com.test.tool",
            "http://localhost:8080",
        ))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::RegisterToolResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn menu_item_invoke_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::invoke_menu_item("menu.item"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::MenuItemResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn menu_item_query_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::query_menu_item("menu.item"))
        .await
        .unwrap();
    match resp.submessage {
        Some(proto::server_originated_message::Submessage::MenuItemResponse(r)) => {
            assert_eq!(r.enabled, Some(true));
            assert_eq!(r.checked, Some(false));
        }
        _ => panic!("wrong submessage"),
    }
    server.shutdown().await;
}

#[tokio::test]
async fn list_prompts_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::list_prompts("s1"))
        .await
        .unwrap();
    match resp.submessage {
        Some(proto::server_originated_message::Submessage::ListPromptsResponse(r)) => {
            assert_eq!(r.unique_prompt_id, vec!["p1", "p2"]);
        }
        _ => panic!("wrong submessage"),
    }
    server.shutdown().await;
}

#[tokio::test]
async fn get_prompt_by_id_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::get_prompt_by_id("s1", "p1"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::GetPromptResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn get_selection_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::get_selection("s1"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::SelectionResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn set_selection_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::set_selection(
            "s1",
            proto::Selection {
                sub_selections: vec![],
            },
        ))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::SelectionResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn set_broadcast_domains_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::set_broadcast_domains(vec![proto::BroadcastDomain {
            session_ids: vec!["s1".to_string()],
        }]))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::SetBroadcastDomainsResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn tmux_send_command_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::tmux_send_command("conn-1", "list-windows"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::TmuxResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn reorder_tabs_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::reorder_tabs(vec![
            proto::reorder_tabs_request::Assignment {
                window_id: Some("w1".to_string()),
                tab_ids: vec!["t1".to_string(), "t2".to_string()],
            },
        ]))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::ReorderTabsResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn get_preference_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::get_preference("SomeKey"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::PreferencesResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn get_color_preset_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::get_color_preset("Solarized Dark"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::ColorPresetResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn open_status_bar_popover_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::open_status_bar_popover(
            "com.test.component",
            "s1",
            "<h1>Hello</h1>",
            200,
            100,
        ))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::StatusBarComponentResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn set_tab_layout_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::set_tab_layout(
            "t1",
            proto::SplitTreeNode {
                vertical: Some(false),
                links: vec![],
            },
        ))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::SetTabLayoutResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn save_arrangement_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::save_arrangement("Test", None))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::SavedArrangementResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn restore_arrangement_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::restore_arrangement("Test", Some("w1")))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::SavedArrangementResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn close_tabs_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::close_tabs(vec!["t1".to_string()], true))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::CloseResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn close_windows_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::close_windows(vec!["w1".to_string()], false))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::CloseResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn activate_tab_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::activate_tab("t1"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::ActivateResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn activate_window_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::activate_window("w1"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::ActivateResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn activate_app_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::activate_app(true, true))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::ActivateResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn get_variable_app_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::get_variable_app(vec!["effectiveTheme".to_string()]))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::VariableResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn get_variable_tab_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::get_variable_tab("t1", vec!["user.foo".to_string()]))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::VariableResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn get_variable_window_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::get_variable_window("w1", vec!["user.bar".to_string()]))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::VariableResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn get_property_session_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::get_property_session("s1", "grid_size"))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::GetPropertyResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn set_property_session_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::set_property_session(
            "s1",
            "grid_size",
            r#"{"width": 120, "height": 40}"#,
        ))
        .await
        .unwrap();
    assert!(matches!(
        resp.submessage,
        Some(proto::server_originated_message::Submessage::SetPropertyResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn list_profiles_round_trip() {
    let server = MockServer::start(mock_server::echo_ok_handler()).await;
    let conn = mock_server::connect_to_mock(server.addr).await;

    let resp = conn
        .call(request::list_profiles(
            vec!["Name".to_string()],
            vec!["some-guid".to_string()],
        ))
        .await
        .unwrap();
    match resp.submessage {
        Some(proto::server_originated_message::Submessage::ListProfilesResponse(r)) => {
            assert_eq!(r.profiles.len(), 1);
        }
        _ => panic!("wrong submessage"),
    }
    server.shutdown().await;
}

// === Additional request builder unit tests (no mock server needed) ===

#[test]
fn get_buffer_with_none_range() {
    let msg = request::get_buffer("s1", None);
    match msg.submessage {
        Some(Submessage::GetBufferRequest(req)) => {
            assert!(req.line_range.is_none());
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn close_tabs_builder() {
    let msg = request::close_tabs(vec!["t1".to_string(), "t2".to_string()], false);
    match msg.submessage {
        Some(Submessage::CloseRequest(req)) => {
            assert_eq!(req.force, Some(false));
            match req.target {
                Some(proto::close_request::Target::Tabs(t)) => {
                    assert_eq!(t.tab_ids, vec!["t1", "t2"]);
                }
                _ => panic!("wrong target"),
            }
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn close_windows_builder() {
    let msg = request::close_windows(vec!["w1".to_string()], true);
    match msg.submessage {
        Some(Submessage::CloseRequest(req)) => {
            assert_eq!(req.force, Some(true));
            match req.target {
                Some(proto::close_request::Target::Windows(w)) => {
                    assert_eq!(w.window_ids, vec!["w1"]);
                }
                _ => panic!("wrong target"),
            }
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn activate_tab_builder() {
    let msg = request::activate_tab("t1");
    match msg.submessage {
        Some(Submessage::ActivateRequest(req)) => {
            assert!(matches!(
                req.identifier,
                Some(proto::activate_request::Identifier::TabId(_))
            ));
            assert_eq!(req.select_tab, Some(true));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn activate_window_builder() {
    let msg = request::activate_window("w1");
    match msg.submessage {
        Some(Submessage::ActivateRequest(req)) => {
            assert!(matches!(
                req.identifier,
                Some(proto::activate_request::Identifier::WindowId(_))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn activate_app_builder() {
    let msg = request::activate_app(true, false);
    match msg.submessage {
        Some(Submessage::ActivateRequest(req)) => {
            assert!(req.identifier.is_none());
            let app = req.activate_app.unwrap();
            assert_eq!(app.raise_all_windows, Some(true));
            assert_eq!(app.ignoring_other_apps, Some(false));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn invoke_function_session_builder() {
    let msg = request::invoke_function_session("s1", "test()");
    match msg.submessage {
        Some(Submessage::InvokeFunctionRequest(req)) => {
            assert!(matches!(
                req.context,
                Some(proto::invoke_function_request::Context::Session(_))
            ));
            assert_eq!(req.invocation.as_deref(), Some("test()"));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn invoke_function_tab_builder() {
    let msg = request::invoke_function_tab("t1", "test()");
    match msg.submessage {
        Some(Submessage::InvokeFunctionRequest(req)) => {
            assert!(matches!(
                req.context,
                Some(proto::invoke_function_request::Context::Tab(_))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn invoke_function_window_builder() {
    let msg = request::invoke_function_window("w1", "test()");
    match msg.submessage {
        Some(Submessage::InvokeFunctionRequest(req)) => {
            assert!(matches!(
                req.context,
                Some(proto::invoke_function_request::Context::Window(_))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn rpc_result_exception_builder() {
    let msg = request::rpc_result_exception("req-1", r#"{"reason":"fail"}"#);
    match msg.submessage {
        Some(Submessage::ServerOriginatedRpcResultRequest(req)) => {
            assert_eq!(req.request_id.as_deref(), Some("req-1"));
            assert!(matches!(
                req.result,
                Some(proto::server_originated_rpc_result_request::Result::JsonException(_))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn set_tab_layout_builder() {
    let msg = request::set_tab_layout(
        "t1",
        proto::SplitTreeNode {
            vertical: Some(true),
            links: vec![],
        },
    );
    match msg.submessage {
        Some(Submessage::SetTabLayoutRequest(req)) => {
            assert_eq!(req.tab_id.as_deref(), Some("t1"));
            assert!(req.root.is_some());
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn set_broadcast_domains_builder() {
    let msg = request::set_broadcast_domains(vec![proto::BroadcastDomain {
        session_ids: vec!["s1".to_string()],
    }]);
    match msg.submessage {
        Some(Submessage::SetBroadcastDomainsRequest(req)) => {
            assert_eq!(req.broadcast_domains.len(), 1);
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn tmux_send_command_builder() {
    let msg = request::tmux_send_command("conn-1", "list-windows");
    match msg.submessage {
        Some(Submessage::TmuxRequest(req)) => {
            assert!(matches!(
                req.payload,
                Some(proto::tmux_request::Payload::SendCommand(_))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn reorder_tabs_builder() {
    let msg = request::reorder_tabs(vec![proto::reorder_tabs_request::Assignment {
        window_id: Some("w1".to_string()),
        tab_ids: vec!["t1".to_string()],
    }]);
    match msg.submessage {
        Some(Submessage::ReorderTabsRequest(req)) => {
            assert_eq!(req.assignments.len(), 1);
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn open_status_bar_popover_builder() {
    let msg = request::open_status_bar_popover("id", "s1", "<h1>Hi</h1>", 300, 200);
    match msg.submessage {
        Some(Submessage::StatusBarComponentRequest(req)) => {
            assert_eq!(req.identifier.as_deref(), Some("id"));
            assert!(req.request.is_some());
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn get_color_preset_builder() {
    let msg = request::get_color_preset("Solarized Dark");
    match msg.submessage {
        Some(Submessage::ColorPresetRequest(req)) => {
            assert!(matches!(
                req.request,
                Some(proto::color_preset_request::Request::GetPreset(_))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn set_selection_builder() {
    let msg = request::set_selection("s1", proto::Selection { sub_selections: vec![] });
    match msg.submessage {
        Some(Submessage::SelectionRequest(req)) => {
            assert!(matches!(
                req.request,
                Some(proto::selection_request::Request::SetSelectionRequest(_))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn save_arrangement_builder() {
    let msg = request::save_arrangement("Test", Some("w1"));
    match msg.submessage {
        Some(Submessage::SavedArrangementRequest(req)) => {
            assert_eq!(req.name.as_deref(), Some("Test"));
            assert_eq!(
                req.action,
                Some(proto::saved_arrangement_request::Action::Save as i32)
            );
            assert_eq!(req.window_id.as_deref(), Some("w1"));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn restore_arrangement_builder() {
    let msg = request::restore_arrangement("Test", None);
    match msg.submessage {
        Some(Submessage::SavedArrangementRequest(req)) => {
            assert_eq!(
                req.action,
                Some(proto::saved_arrangement_request::Action::Restore as i32)
            );
            assert!(req.window_id.is_none());
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn get_prompt_builder() {
    let msg = request::get_prompt("s1");
    match msg.submessage {
        Some(Submessage::GetPromptRequest(req)) => {
            assert_eq!(req.session.as_deref(), Some("s1"));
            assert!(req.unique_prompt_id.is_none());
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn get_prompt_by_id_builder() {
    let msg = request::get_prompt_by_id("s1", "p1");
    match msg.submessage {
        Some(Submessage::GetPromptRequest(req)) => {
            assert_eq!(req.session.as_deref(), Some("s1"));
            assert_eq!(req.unique_prompt_id.as_deref(), Some("p1"));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn list_prompts_builder() {
    let msg = request::list_prompts("s1");
    match msg.submessage {
        Some(Submessage::ListPromptsRequest(req)) => {
            assert_eq!(req.session.as_deref(), Some("s1"));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn get_variable_app_builder() {
    let msg = request::get_variable_app(vec!["effectiveTheme".to_string()]);
    match msg.submessage {
        Some(Submessage::VariableRequest(req)) => {
            assert!(matches!(
                req.scope,
                Some(proto::variable_request::Scope::App(true))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn get_variable_tab_builder() {
    let msg = request::get_variable_tab("t1", vec!["user.foo".to_string()]);
    match msg.submessage {
        Some(Submessage::VariableRequest(req)) => {
            assert!(matches!(
                req.scope,
                Some(proto::variable_request::Scope::TabId(_))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn get_variable_window_builder() {
    let msg = request::get_variable_window("w1", vec!["user.bar".to_string()]);
    match msg.submessage {
        Some(Submessage::VariableRequest(req)) => {
            assert!(matches!(
                req.scope,
                Some(proto::variable_request::Scope::WindowId(_))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn get_property_session_builder() {
    let msg = request::get_property_session("s1", "grid_size");
    match msg.submessage {
        Some(Submessage::GetPropertyRequest(req)) => {
            assert!(matches!(
                req.identifier,
                Some(proto::get_property_request::Identifier::SessionId(_))
            ));
            assert_eq!(req.name.as_deref(), Some("grid_size"));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn set_property_session_builder() {
    let msg = request::set_property_session("s1", "buried", "true");
    match msg.submessage {
        Some(Submessage::SetPropertyRequest(req)) => {
            assert!(matches!(
                req.identifier,
                Some(proto::set_property_request::Identifier::SessionId(_))
            ));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn set_profile_property_session_builder() {
    let msg = request::set_profile_property_session("s1", "Name", r#""New Name""#);
    match msg.submessage {
        Some(Submessage::SetProfilePropertyRequest(req)) => {
            assert!(matches!(
                req.target,
                Some(proto::set_profile_property_request::Target::Session(_))
            ));
            assert_eq!(req.key.as_deref(), Some("Name"));
        }
        _ => panic!("wrong submessage"),
    }
}

#[test]
fn restart_session_builder() {
    let msg = request::restart_session("s1", false);
    match msg.submessage {
        Some(Submessage::RestartSessionRequest(req)) => {
            assert_eq!(req.session_id.as_deref(), Some("s1"));
            assert_eq!(req.only_if_exited, Some(false));
        }
        _ => panic!("wrong submessage"),
    }
}
