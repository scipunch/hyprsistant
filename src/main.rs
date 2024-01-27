mod common;
mod layout_per_workspace;
mod workspace_number;

use hyprland::{self, event_listener::EventListener};

fn main() {
    let mut listener = EventListener::new();

    layout_per_workspace::add_handler(&mut listener);
    workspace_number::add_handler(&mut listener);

    listener
        .start_listener()
        .expect("Failed to start event listener");
}
