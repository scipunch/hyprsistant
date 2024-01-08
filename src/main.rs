use hyprland::{
    self,
    ctl::switch_xkb_layout,
    data::Workspace,
    event_listener::EventListener,
    shared::{HyprDataActive, WorkspaceType},
};
use std::{
    io,
    process::{Command, Output},
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum KeyboardLayout {
    En,
    Ru,
}

impl KeyboardLayout {
    pub fn as_id(&self) -> u8 {
        use KeyboardLayout::*;
        match self {
            En => 0,
            Ru => 1,
        }
    }
}

impl std::ops::Not for KeyboardLayout {
    type Output = KeyboardLayout;

    fn not(self) -> Self::Output {
        use KeyboardLayout::*;
        match self {
            Ru => En,
            En => Ru,
        }
    }
}

impl FromStr for KeyboardLayout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Russian" => Ok(KeyboardLayout::Ru),
            "English (US)" => Ok(KeyboardLayout::En),
            _ => Err(()),
        }
    }
}

fn main() {
    let mut listener = EventListener::new();

    add_layout_display_manager(&mut listener, KeyboardLayout::Ru);
    add_layout_per_workspace_manager(&mut listener);
    add_current_workspace_number(&mut listener);

    listener.start_listener().unwrap();
}

fn add_layout_display_manager(listener: &mut EventListener, layout_to_display: KeyboardLayout) {
    listener.add_keyboard_layout_change_handler(move |event| {
        if let Some((_, current_layout)) = event.keyboard_name.split_once(',') {
            let _ = KeyboardLayout::from_str(current_layout).map(|layout| {
                if layout == layout_to_display {
                    let _ = update_eww_var("left_section_style", "left_section")
                        .and(update_eww_var("layout_alert_reveal", "true"));
                } else {
                    let _ = update_eww_var("layout_alert_reveal", "false")
                        .and(update_eww_var("left_section_style", "section"));
                }
            });
        }
    });
}

fn add_layout_per_workspace_manager(listener: &mut EventListener) {
    let workspaces = Arc::new(Mutex::new(vec![]));
    let keyboard_name = Arc::new(Mutex::new(String::new()));

    let kb_workspaces = Arc::clone(&workspaces);
    let kb_keyboard_name = Arc::clone(&keyboard_name);
    listener.add_keyboard_layout_change_handler(move |event| {
        let mut workspaces = kb_workspaces.lock().unwrap();

        let current_workspace_idx = Workspace::get_active().unwrap().id as usize - 1;
        if current_workspace_idx >= workspaces.len() {
            for _ in workspaces.len()..current_workspace_idx + 1 {
                workspaces.push(KeyboardLayout::En);
            }
        }

        let (kb_name, layout_name) = event.keyboard_name.split_once(',').unwrap();
        let mut keyboard_name = kb_keyboard_name.lock().unwrap();
        *keyboard_name = kb_name.to_string();

        workspaces[current_workspace_idx] = KeyboardLayout::from_str(layout_name).unwrap();
    });

    let wc_workspaces = Arc::clone(&workspaces);
    let wc_keyboard_name = Arc::clone(&keyboard_name);
    listener.add_workspace_change_handler(move |event| {
        let workspace_name = match event {
            WorkspaceType::Regular(name) => name,
            WorkspaceType::Special(Some(name)) => name,
            _ => String::new(),
        };
        let workspace_idx = workspace_name.parse::<usize>().unwrap() - 1;

        let mut workspaces = wc_workspaces.lock().unwrap();
        if workspace_idx >= workspaces.len() {
            for _ in workspaces.len()..workspace_idx + 1 {
                workspaces.push(KeyboardLayout::En);
            }
        }
        let keyboard_name = wc_keyboard_name.lock().unwrap();
        let _ = switch_xkb_layout::call(
            &keyboard_name,
            hyprland::ctl::switch_xkb_layout::SwitchXKBLayoutCmdTypes::Id(
                workspaces[workspace_idx].as_id(),
            ),
        );
    })
}

fn add_current_workspace_number(listener: &mut EventListener) {
    listener.add_workspace_change_handler(|event| {
        if let WorkspaceType::Regular(name) = event {
            let _ = update_eww_var("current_workspace_name", &name);
        }
    })
}

fn update_eww_var(var: &str, value: &str) -> io::Result<Output> {
    Command::new("eww")
        .arg("update")
        .arg(format!("{var}={value}"))
        .output()
}
