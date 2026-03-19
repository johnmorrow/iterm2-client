use crate::proto;
use crate::proto::client_originated_message::Submessage;

fn wrap(submessage: Submessage) -> proto::ClientOriginatedMessage {
    proto::ClientOriginatedMessage {
        id: None, // Connection::call() sets this
        submessage: Some(submessage),
    }
}

// --- Session operations ---

pub fn list_sessions() -> proto::ClientOriginatedMessage {
    wrap(Submessage::ListSessionsRequest(proto::ListSessionsRequest {}))
}

pub fn send_text(session_id: &str, text: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SendTextRequest(proto::SendTextRequest {
        session: Some(session_id.to_string()),
        text: Some(text.to_string()),
        suppress_broadcast: None,
    }))
}

pub fn get_buffer(
    session_id: &str,
    line_range: Option<proto::LineRange>,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::GetBufferRequest(proto::GetBufferRequest {
        session: Some(session_id.to_string()),
        line_range,
        include_styles: Some(false),
    }))
}

pub fn get_buffer_trailing(session_id: &str, lines: i32) -> proto::ClientOriginatedMessage {
    get_buffer(
        session_id,
        Some(proto::LineRange {
            screen_contents_only: None,
            trailing_lines: Some(lines),
            windowed_coord_range: None,
        }),
    )
}

pub fn get_buffer_screen(session_id: &str) -> proto::ClientOriginatedMessage {
    get_buffer(
        session_id,
        Some(proto::LineRange {
            screen_contents_only: Some(true),
            trailing_lines: None,
            windowed_coord_range: None,
        }),
    )
}

pub fn get_prompt(session_id: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::GetPromptRequest(proto::GetPromptRequest {
        session: Some(session_id.to_string()),
        unique_prompt_id: None,
    }))
}

pub fn get_prompt_by_id(session_id: &str, prompt_id: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::GetPromptRequest(proto::GetPromptRequest {
        session: Some(session_id.to_string()),
        unique_prompt_id: Some(prompt_id.to_string()),
    }))
}

pub fn list_prompts(session_id: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::ListPromptsRequest(proto::ListPromptsRequest {
        session: Some(session_id.to_string()),
        first_unique_id: None,
        last_unique_id: None,
    }))
}

// --- Tab operations ---

pub fn create_tab(
    profile_name: Option<&str>,
    window_id: Option<&str>,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::CreateTabRequest(proto::CreateTabRequest {
        profile_name: profile_name.map(|s| s.to_string()),
        window_id: window_id.map(|s| s.to_string()),
        tab_index: None,
        #[allow(deprecated)]
        command: None,
        custom_profile_properties: vec![],
    }))
}

pub fn split_pane(
    session_id: &str,
    direction: proto::split_pane_request::SplitDirection,
    before: bool,
    profile_name: Option<&str>,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SplitPaneRequest(proto::SplitPaneRequest {
        session: Some(session_id.to_string()),
        split_direction: Some(direction.into()),
        before: Some(before),
        profile_name: profile_name.map(|s| s.to_string()),
        custom_profile_properties: vec![],
    }))
}

// --- Profile operations ---

pub fn get_profile_property(
    session_id: &str,
    keys: Vec<String>,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::GetProfilePropertyRequest(
        proto::GetProfilePropertyRequest {
            session: Some(session_id.to_string()),
            keys,
        },
    ))
}

pub fn set_profile_property_session(
    session_id: &str,
    key: &str,
    json_value: &str,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SetProfilePropertyRequest(
        proto::SetProfilePropertyRequest {
            target: Some(proto::set_profile_property_request::Target::Session(
                session_id.to_string(),
            )),
            key: Some(key.to_string()),
            json_value: Some(json_value.to_string()),
            assignments: vec![],
        },
    ))
}

pub fn list_profiles(
    properties: Vec<String>,
    guids: Vec<String>,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::ListProfilesRequest(proto::ListProfilesRequest {
        properties,
        guids,
    }))
}

// --- Property operations ---

pub fn get_property_window(window_id: &str, name: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::GetPropertyRequest(proto::GetPropertyRequest {
        identifier: Some(proto::get_property_request::Identifier::WindowId(
            window_id.to_string(),
        )),
        name: Some(name.to_string()),
    }))
}

pub fn get_property_session(session_id: &str, name: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::GetPropertyRequest(proto::GetPropertyRequest {
        identifier: Some(proto::get_property_request::Identifier::SessionId(
            session_id.to_string(),
        )),
        name: Some(name.to_string()),
    }))
}

pub fn set_property_window(
    window_id: &str,
    name: &str,
    json_value: &str,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SetPropertyRequest(proto::SetPropertyRequest {
        identifier: Some(proto::set_property_request::Identifier::WindowId(
            window_id.to_string(),
        )),
        name: Some(name.to_string()),
        json_value: Some(json_value.to_string()),
    }))
}

pub fn set_property_session(
    session_id: &str,
    name: &str,
    json_value: &str,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SetPropertyRequest(proto::SetPropertyRequest {
        identifier: Some(proto::set_property_request::Identifier::SessionId(
            session_id.to_string(),
        )),
        name: Some(name.to_string()),
        json_value: Some(json_value.to_string()),
    }))
}

// --- Variable operations ---

pub fn get_variable_session(session_id: &str, names: Vec<String>) -> proto::ClientOriginatedMessage {
    wrap(Submessage::VariableRequest(proto::VariableRequest {
        scope: Some(proto::variable_request::Scope::SessionId(
            session_id.to_string(),
        )),
        set: vec![],
        get: names,
    }))
}

pub fn set_variable_session(
    session_id: &str,
    sets: Vec<(String, String)>,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::VariableRequest(proto::VariableRequest {
        scope: Some(proto::variable_request::Scope::SessionId(
            session_id.to_string(),
        )),
        set: sets
            .into_iter()
            .map(|(name, value)| proto::variable_request::Set {
                name: Some(name),
                value: Some(value),
            })
            .collect(),
        get: vec![],
    }))
}

pub fn get_variable_app(names: Vec<String>) -> proto::ClientOriginatedMessage {
    wrap(Submessage::VariableRequest(proto::VariableRequest {
        scope: Some(proto::variable_request::Scope::App(true)),
        set: vec![],
        get: names,
    }))
}

pub fn get_variable_tab(tab_id: &str, names: Vec<String>) -> proto::ClientOriginatedMessage {
    wrap(Submessage::VariableRequest(proto::VariableRequest {
        scope: Some(proto::variable_request::Scope::TabId(tab_id.to_string())),
        set: vec![],
        get: names,
    }))
}

pub fn get_variable_window(window_id: &str, names: Vec<String>) -> proto::ClientOriginatedMessage {
    wrap(Submessage::VariableRequest(proto::VariableRequest {
        scope: Some(proto::variable_request::Scope::WindowId(window_id.to_string())),
        set: vec![],
        get: names,
    }))
}

// --- Activate operations ---

pub fn activate_session(session_id: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::ActivateRequest(proto::ActivateRequest {
        identifier: Some(proto::activate_request::Identifier::SessionId(
            session_id.to_string(),
        )),
        order_window_front: Some(true),
        select_tab: Some(true),
        select_session: Some(true),
        activate_app: None,
    }))
}

pub fn activate_tab(tab_id: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::ActivateRequest(proto::ActivateRequest {
        identifier: Some(proto::activate_request::Identifier::TabId(
            tab_id.to_string(),
        )),
        order_window_front: Some(true),
        select_tab: Some(true),
        select_session: None,
        activate_app: None,
    }))
}

pub fn activate_window(window_id: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::ActivateRequest(proto::ActivateRequest {
        identifier: Some(proto::activate_request::Identifier::WindowId(
            window_id.to_string(),
        )),
        order_window_front: Some(true),
        select_tab: None,
        select_session: None,
        activate_app: None,
    }))
}

pub fn activate_app(raise_all: bool, ignoring_other_apps: bool) -> proto::ClientOriginatedMessage {
    wrap(Submessage::ActivateRequest(proto::ActivateRequest {
        identifier: None,
        order_window_front: None,
        select_tab: None,
        select_session: None,
        activate_app: Some(proto::activate_request::App {
            raise_all_windows: Some(raise_all),
            ignoring_other_apps: Some(ignoring_other_apps),
        }),
    }))
}

// --- Transaction operations ---

pub fn begin_transaction() -> proto::ClientOriginatedMessage {
    wrap(Submessage::TransactionRequest(proto::TransactionRequest {
        begin: Some(true),
    }))
}

pub fn end_transaction() -> proto::ClientOriginatedMessage {
    wrap(Submessage::TransactionRequest(proto::TransactionRequest {
        begin: Some(false),
    }))
}

// --- Notification subscription ---

pub fn subscribe_notification(
    notification_type: proto::NotificationType,
    session_id: Option<&str>,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::NotificationRequest(proto::NotificationRequest {
        session: session_id.map(|s| s.to_string()),
        subscribe: Some(true),
        notification_type: Some(notification_type.into()),
        arguments: None,
    }))
}

pub fn unsubscribe_notification(
    notification_type: proto::NotificationType,
    session_id: Option<&str>,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::NotificationRequest(proto::NotificationRequest {
        session: session_id.map(|s| s.to_string()),
        subscribe: Some(false),
        notification_type: Some(notification_type.into()),
        arguments: None,
    }))
}

// --- Inject ---

pub fn inject(session_ids: Vec<String>, data: Vec<u8>) -> proto::ClientOriginatedMessage {
    wrap(Submessage::InjectRequest(proto::InjectRequest {
        session_id: session_ids,
        data: Some(data),
    }))
}

// --- Close ---

pub fn close_sessions(session_ids: Vec<String>, force: bool) -> proto::ClientOriginatedMessage {
    wrap(Submessage::CloseRequest(proto::CloseRequest {
        target: Some(proto::close_request::Target::Sessions(
            proto::close_request::CloseSessions { session_ids },
        )),
        force: Some(force),
    }))
}

pub fn close_tabs(tab_ids: Vec<String>, force: bool) -> proto::ClientOriginatedMessage {
    wrap(Submessage::CloseRequest(proto::CloseRequest {
        target: Some(proto::close_request::Target::Tabs(
            proto::close_request::CloseTabs { tab_ids },
        )),
        force: Some(force),
    }))
}

pub fn close_windows(window_ids: Vec<String>, force: bool) -> proto::ClientOriginatedMessage {
    wrap(Submessage::CloseRequest(proto::CloseRequest {
        target: Some(proto::close_request::Target::Windows(
            proto::close_request::CloseWindows { window_ids },
        )),
        force: Some(force),
    }))
}

// --- Focus ---

pub fn focus() -> proto::ClientOriginatedMessage {
    wrap(Submessage::FocusRequest(proto::FocusRequest {}))
}

// --- Saved arrangements ---

pub fn restore_arrangement(name: &str, window_id: Option<&str>) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SavedArrangementRequest(
        proto::SavedArrangementRequest {
            name: Some(name.to_string()),
            action: Some(proto::saved_arrangement_request::Action::Restore.into()),
            window_id: window_id.map(|s| s.to_string()),
        },
    ))
}

pub fn save_arrangement(name: &str, window_id: Option<&str>) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SavedArrangementRequest(
        proto::SavedArrangementRequest {
            name: Some(name.to_string()),
            action: Some(proto::saved_arrangement_request::Action::Save.into()),
            window_id: window_id.map(|s| s.to_string()),
        },
    ))
}

pub fn list_arrangements() -> proto::ClientOriginatedMessage {
    wrap(Submessage::SavedArrangementRequest(
        proto::SavedArrangementRequest {
            name: None,
            action: Some(proto::saved_arrangement_request::Action::List.into()),
            window_id: None,
        },
    ))
}

// --- Menu item ---

pub fn invoke_menu_item(identifier: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::MenuItemRequest(proto::MenuItemRequest {
        identifier: Some(identifier.to_string()),
        query_only: Some(false),
    }))
}

pub fn query_menu_item(identifier: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::MenuItemRequest(proto::MenuItemRequest {
        identifier: Some(identifier.to_string()),
        query_only: Some(true),
    }))
}

// --- Restart session ---

pub fn restart_session(session_id: &str, only_if_exited: bool) -> proto::ClientOriginatedMessage {
    wrap(Submessage::RestartSessionRequest(
        proto::RestartSessionRequest {
            session_id: Some(session_id.to_string()),
            only_if_exited: Some(only_if_exited),
        },
    ))
}

// --- Register tool ---

pub fn register_tool(
    name: &str,
    identifier: &str,
    url: &str,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::RegisterToolRequest(proto::RegisterToolRequest {
        name: Some(name.to_string()),
        identifier: Some(identifier.to_string()),
        tool_type: Some(proto::register_tool_request::ToolType::WebViewTool.into()),
        reveal_if_already_registered: Some(false),
        url: Some(url.to_string()),
    }))
}

// --- Set tab layout ---

pub fn set_tab_layout(
    tab_id: &str,
    root: proto::SplitTreeNode,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SetTabLayoutRequest(proto::SetTabLayoutRequest {
        tab_id: Some(tab_id.to_string()),
        root: Some(root),
    }))
}

// --- Broadcast domains ---

pub fn get_broadcast_domains() -> proto::ClientOriginatedMessage {
    wrap(Submessage::GetBroadcastDomainsRequest(
        proto::GetBroadcastDomainsRequest {},
    ))
}

pub fn set_broadcast_domains(
    domains: Vec<proto::BroadcastDomain>,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SetBroadcastDomainsRequest(
        proto::SetBroadcastDomainsRequest {
            broadcast_domains: domains,
        },
    ))
}

// --- Tmux ---

pub fn tmux_list_connections() -> proto::ClientOriginatedMessage {
    wrap(Submessage::TmuxRequest(proto::TmuxRequest {
        payload: Some(proto::tmux_request::Payload::ListConnections(
            proto::tmux_request::ListConnections {},
        )),
    }))
}

pub fn tmux_send_command(
    connection_id: &str,
    command: &str,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::TmuxRequest(proto::TmuxRequest {
        payload: Some(proto::tmux_request::Payload::SendCommand(
            proto::tmux_request::SendCommand {
                connection_id: Some(connection_id.to_string()),
                command: Some(command.to_string()),
            },
        )),
    }))
}

// --- Reorder tabs ---

pub fn reorder_tabs(
    assignments: Vec<proto::reorder_tabs_request::Assignment>,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::ReorderTabsRequest(proto::ReorderTabsRequest {
        assignments,
    }))
}

// --- Preferences ---

pub fn get_preference(key: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::PreferencesRequest(proto::PreferencesRequest {
        requests: vec![proto::preferences_request::Request {
            request: Some(
                proto::preferences_request::request::Request::GetPreferenceRequest(
                    proto::preferences_request::request::GetPreference {
                        key: Some(key.to_string()),
                    },
                ),
            ),
        }],
    }))
}

// --- Color presets ---

pub fn list_color_presets() -> proto::ClientOriginatedMessage {
    wrap(Submessage::ColorPresetRequest(proto::ColorPresetRequest {
        request: Some(proto::color_preset_request::Request::ListPresets(
            proto::color_preset_request::ListPresets {},
        )),
    }))
}

pub fn get_color_preset(name: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::ColorPresetRequest(proto::ColorPresetRequest {
        request: Some(proto::color_preset_request::Request::GetPreset(
            proto::color_preset_request::GetPreset {
                name: Some(name.to_string()),
            },
        )),
    }))
}

// --- Selection ---

pub fn get_selection(session_id: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SelectionRequest(proto::SelectionRequest {
        request: Some(proto::selection_request::Request::GetSelectionRequest(
            proto::selection_request::GetSelectionRequest {
                session_id: Some(session_id.to_string()),
            },
        )),
    }))
}

pub fn set_selection(
    session_id: &str,
    selection: proto::Selection,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::SelectionRequest(proto::SelectionRequest {
        request: Some(proto::selection_request::Request::SetSelectionRequest(
            proto::selection_request::SetSelectionRequest {
                session_id: Some(session_id.to_string()),
                selection: Some(selection),
            },
        )),
    }))
}

// --- Status bar component ---

pub fn open_status_bar_popover(
    identifier: &str,
    session_id: &str,
    html: &str,
    width: i32,
    height: i32,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::StatusBarComponentRequest(
        proto::StatusBarComponentRequest {
            request: Some(
                proto::status_bar_component_request::Request::OpenPopover(
                    proto::status_bar_component_request::OpenPopover {
                        session_id: Some(session_id.to_string()),
                        html: Some(html.to_string()),
                        size: Some(proto::Size {
                            width: Some(width),
                            height: Some(height),
                        }),
                    },
                ),
            ),
            identifier: Some(identifier.to_string()),
        },
    ))
}

// --- Invoke function ---

pub fn invoke_function_app(invocation: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::InvokeFunctionRequest(
        proto::InvokeFunctionRequest {
            context: Some(proto::invoke_function_request::Context::App(
                proto::invoke_function_request::App {},
            )),
            invocation: Some(invocation.to_string()),
            timeout: None,
        },
    ))
}

pub fn invoke_function_session(
    session_id: &str,
    invocation: &str,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::InvokeFunctionRequest(
        proto::InvokeFunctionRequest {
            context: Some(proto::invoke_function_request::Context::Session(
                proto::invoke_function_request::Session {
                    session_id: Some(session_id.to_string()),
                },
            )),
            invocation: Some(invocation.to_string()),
            timeout: None,
        },
    ))
}

pub fn invoke_function_tab(tab_id: &str, invocation: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::InvokeFunctionRequest(
        proto::InvokeFunctionRequest {
            context: Some(proto::invoke_function_request::Context::Tab(
                proto::invoke_function_request::Tab {
                    tab_id: Some(tab_id.to_string()),
                },
            )),
            invocation: Some(invocation.to_string()),
            timeout: None,
        },
    ))
}

pub fn invoke_function_window(
    window_id: &str,
    invocation: &str,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::InvokeFunctionRequest(
        proto::InvokeFunctionRequest {
            context: Some(proto::invoke_function_request::Context::Window(
                proto::invoke_function_request::Window {
                    window_id: Some(window_id.to_string()),
                },
            )),
            invocation: Some(invocation.to_string()),
            timeout: None,
        },
    ))
}

// --- Server-originated RPC result ---

pub fn rpc_result_value(request_id: &str, json_value: &str) -> proto::ClientOriginatedMessage {
    wrap(Submessage::ServerOriginatedRpcResultRequest(
        proto::ServerOriginatedRpcResultRequest {
            request_id: Some(request_id.to_string()),
            result: Some(
                proto::server_originated_rpc_result_request::Result::JsonValue(
                    json_value.to_string(),
                ),
            ),
        },
    ))
}

pub fn rpc_result_exception(
    request_id: &str,
    json_exception: &str,
) -> proto::ClientOriginatedMessage {
    wrap(Submessage::ServerOriginatedRpcResultRequest(
        proto::ServerOriginatedRpcResultRequest {
            request_id: Some(request_id.to_string()),
            result: Some(
                proto::server_originated_rpc_result_request::Result::JsonException(
                    json_exception.to_string(),
                ),
            ),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::client_originated_message::Submessage;

    #[test]
    fn list_sessions_request() {
        let msg = list_sessions();
        assert!(msg.id.is_none());
        assert!(matches!(
            msg.submessage,
            Some(Submessage::ListSessionsRequest(_))
        ));
    }

    #[test]
    fn send_text_request() {
        let msg = send_text("session-123", "hello\n");
        match msg.submessage {
            Some(Submessage::SendTextRequest(req)) => {
                assert_eq!(req.session.as_deref(), Some("session-123"));
                assert_eq!(req.text.as_deref(), Some("hello\n"));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn get_buffer_trailing_request() {
        let msg = get_buffer_trailing("s1", 50);
        match msg.submessage {
            Some(Submessage::GetBufferRequest(req)) => {
                assert_eq!(req.session.as_deref(), Some("s1"));
                assert_eq!(req.line_range.unwrap().trailing_lines, Some(50));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn get_buffer_screen_request() {
        let msg = get_buffer_screen("s1");
        match msg.submessage {
            Some(Submessage::GetBufferRequest(req)) => {
                assert_eq!(
                    req.line_range.unwrap().screen_contents_only,
                    Some(true)
                );
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn create_tab_request() {
        let msg = create_tab(Some("Default"), Some("w1"));
        match msg.submessage {
            Some(Submessage::CreateTabRequest(req)) => {
                assert_eq!(req.profile_name.as_deref(), Some("Default"));
                assert_eq!(req.window_id.as_deref(), Some("w1"));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn split_pane_request() {
        let msg = split_pane(
            "s1",
            proto::split_pane_request::SplitDirection::Vertical,
            false,
            None,
        );
        match msg.submessage {
            Some(Submessage::SplitPaneRequest(req)) => {
                assert_eq!(req.session.as_deref(), Some("s1"));
                assert_eq!(req.split_direction, Some(0)); // VERTICAL = 0
                assert_eq!(req.before, Some(false));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn get_profile_property_request() {
        let msg = get_profile_property("s1", vec!["Name".to_string()]);
        match msg.submessage {
            Some(Submessage::GetProfilePropertyRequest(req)) => {
                assert_eq!(req.session.as_deref(), Some("s1"));
                assert_eq!(req.keys, vec!["Name".to_string()]);
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn set_profile_property_request() {
        let msg = set_profile_property_session("s1", "Badge Text", r#""hello""#);
        match msg.submessage {
            Some(Submessage::SetProfilePropertyRequest(req)) => {
                assert_eq!(req.key.as_deref(), Some("Badge Text"));
                assert_eq!(req.json_value.as_deref(), Some(r#""hello""#));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn variable_get_session_request() {
        let msg = get_variable_session("s1", vec!["user.foo".to_string()]);
        match msg.submessage {
            Some(Submessage::VariableRequest(req)) => {
                assert_eq!(req.get, vec!["user.foo".to_string()]);
                assert!(req.set.is_empty());
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn variable_set_session_request() {
        let msg = set_variable_session(
            "s1",
            vec![("user.foo".to_string(), r#""bar""#.to_string())],
        );
        match msg.submessage {
            Some(Submessage::VariableRequest(req)) => {
                assert_eq!(req.set.len(), 1);
                assert_eq!(req.set[0].name.as_deref(), Some("user.foo"));
                assert_eq!(req.set[0].value.as_deref(), Some(r#""bar""#));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn transaction_begin_end() {
        let begin = begin_transaction();
        match begin.submessage {
            Some(Submessage::TransactionRequest(req)) => {
                assert_eq!(req.begin, Some(true));
            }
            _ => panic!("wrong submessage"),
        }

        let end = end_transaction();
        match end.submessage {
            Some(Submessage::TransactionRequest(req)) => {
                assert_eq!(req.begin, Some(false));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn subscribe_unsubscribe_notification() {
        let sub = subscribe_notification(
            proto::NotificationType::NotifyOnNewSession,
            None,
        );
        match sub.submessage {
            Some(Submessage::NotificationRequest(req)) => {
                assert_eq!(req.subscribe, Some(true));
                assert_eq!(
                    req.notification_type,
                    Some(proto::NotificationType::NotifyOnNewSession as i32)
                );
            }
            _ => panic!("wrong submessage"),
        }

        let unsub = unsubscribe_notification(
            proto::NotificationType::NotifyOnNewSession,
            None,
        );
        match unsub.submessage {
            Some(Submessage::NotificationRequest(req)) => {
                assert_eq!(req.subscribe, Some(false));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn close_sessions_request() {
        let msg = close_sessions(vec!["s1".to_string(), "s2".to_string()], true);
        match msg.submessage {
            Some(Submessage::CloseRequest(req)) => {
                assert_eq!(req.force, Some(true));
                match req.target {
                    Some(proto::close_request::Target::Sessions(s)) => {
                        assert_eq!(s.session_ids, vec!["s1", "s2"]);
                    }
                    _ => panic!("wrong target"),
                }
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn focus_request() {
        let msg = focus();
        assert!(matches!(
            msg.submessage,
            Some(Submessage::FocusRequest(_))
        ));
    }

    #[test]
    fn activate_session_request() {
        let msg = activate_session("s1");
        match msg.submessage {
            Some(Submessage::ActivateRequest(req)) => {
                assert!(matches!(
                    req.identifier,
                    Some(proto::activate_request::Identifier::SessionId(_))
                ));
                assert_eq!(req.order_window_front, Some(true));
                assert_eq!(req.select_tab, Some(true));
                assert_eq!(req.select_session, Some(true));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn inject_request() {
        let msg = inject(vec!["s1".to_string()], b"hello".to_vec());
        match msg.submessage {
            Some(Submessage::InjectRequest(req)) => {
                assert_eq!(req.session_id, vec!["s1"]);
                assert_eq!(req.data.as_deref(), Some(b"hello".as_slice()));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn menu_item_invoke_and_query() {
        let msg = invoke_menu_item("menu.item.1");
        match msg.submessage {
            Some(Submessage::MenuItemRequest(req)) => {
                assert_eq!(req.identifier.as_deref(), Some("menu.item.1"));
                assert_eq!(req.query_only, Some(false));
            }
            _ => panic!("wrong submessage"),
        }

        let msg = query_menu_item("menu.item.1");
        match msg.submessage {
            Some(Submessage::MenuItemRequest(req)) => {
                assert_eq!(req.query_only, Some(true));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn restart_session_request() {
        let msg = restart_session("s1", true);
        match msg.submessage {
            Some(Submessage::RestartSessionRequest(req)) => {
                assert_eq!(req.session_id.as_deref(), Some("s1"));
                assert_eq!(req.only_if_exited, Some(true));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn saved_arrangement_list() {
        let msg = list_arrangements();
        match msg.submessage {
            Some(Submessage::SavedArrangementRequest(req)) => {
                assert_eq!(
                    req.action,
                    Some(proto::saved_arrangement_request::Action::List as i32)
                );
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn broadcast_domains_request() {
        let msg = get_broadcast_domains();
        assert!(matches!(
            msg.submessage,
            Some(Submessage::GetBroadcastDomainsRequest(_))
        ));
    }

    #[test]
    fn tmux_list_request() {
        let msg = tmux_list_connections();
        match msg.submessage {
            Some(Submessage::TmuxRequest(req)) => {
                assert!(matches!(
                    req.payload,
                    Some(proto::tmux_request::Payload::ListConnections(_))
                ));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn list_color_presets_request() {
        let msg = list_color_presets();
        match msg.submessage {
            Some(Submessage::ColorPresetRequest(req)) => {
                assert!(matches!(
                    req.request,
                    Some(proto::color_preset_request::Request::ListPresets(_))
                ));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn get_selection_request() {
        let msg = get_selection("s1");
        match msg.submessage {
            Some(Submessage::SelectionRequest(req)) => {
                assert!(matches!(
                    req.request,
                    Some(proto::selection_request::Request::GetSelectionRequest(_))
                ));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn invoke_function_app_request() {
        let msg = invoke_function_app("test()");
        match msg.submessage {
            Some(Submessage::InvokeFunctionRequest(req)) => {
                assert_eq!(req.invocation.as_deref(), Some("test()"));
                assert!(matches!(
                    req.context,
                    Some(proto::invoke_function_request::Context::App(_))
                ));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn rpc_result_value_request() {
        let msg = rpc_result_value("req-1", r#""ok""#);
        match msg.submessage {
            Some(Submessage::ServerOriginatedRpcResultRequest(req)) => {
                assert_eq!(req.request_id.as_deref(), Some("req-1"));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn list_profiles_request() {
        let msg = list_profiles(vec![], vec![]);
        assert!(matches!(
            msg.submessage,
            Some(Submessage::ListProfilesRequest(_))
        ));
    }

    #[test]
    fn get_preference_request() {
        let msg = get_preference("SomeKey");
        assert!(matches!(
            msg.submessage,
            Some(Submessage::PreferencesRequest(_))
        ));
    }

    #[test]
    fn register_tool_request() {
        let msg = register_tool("My Tool", "com.example.tool", "http://localhost:8080");
        match msg.submessage {
            Some(Submessage::RegisterToolRequest(req)) => {
                assert_eq!(req.name.as_deref(), Some("My Tool"));
                assert_eq!(req.identifier.as_deref(), Some("com.example.tool"));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn get_property_window_request() {
        let msg = get_property_window("w1", "frame");
        match msg.submessage {
            Some(Submessage::GetPropertyRequest(req)) => {
                assert_eq!(req.name.as_deref(), Some("frame"));
                assert!(matches!(
                    req.identifier,
                    Some(proto::get_property_request::Identifier::WindowId(_))
                ));
            }
            _ => panic!("wrong submessage"),
        }
    }

    #[test]
    fn set_property_window_request() {
        let msg = set_property_window("w1", "fullscreen", "true");
        match msg.submessage {
            Some(Submessage::SetPropertyRequest(req)) => {
                assert_eq!(req.name.as_deref(), Some("fullscreen"));
                assert_eq!(req.json_value.as_deref(), Some("true"));
            }
            _ => panic!("wrong submessage"),
        }
    }
}
