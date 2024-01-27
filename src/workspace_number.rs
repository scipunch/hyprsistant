use hyprland::{event_listener::EventListener, shared::WorkspaceType};

use crate::common;

pub fn add_handler(listener: &mut EventListener) {
    listener.add_workspace_change_handler(|event| {
        if let WorkspaceType::Regular(name) = event {
            let _ = common::update_eww_var("current_workspace_name", &name);
        }
    })
}
