use std::{
    io,
    process::{Command, Output},
};

pub fn update_eww_var(var: &str, value: &str) -> io::Result<Output> {
    Command::new("eww")
        .arg("update")
        .arg(format!("{var}={value}"))
        .output()
}
