#![feature(lazy_cell)]

mod config;
mod keyboard_mouse_events;
mod read_memory;
mod resolution;
mod statics;

pub use read_memory::get_toughness;
pub use resolution::get_resolution;
pub use statics::RESOLUTION;
pub use statics::TOUGHNESS_UPDATE_INTERVAL;

pub use config::CONFIG;

pub use keyboard_mouse_events::{is_key_event, is_mouse_event};

pub fn error_to_cmd(message: &str) {
    std::process::Command::new("cmd")
        .args(["/C", "echo", message, "&", "pause"])
        .status()
        .unwrap();
}
