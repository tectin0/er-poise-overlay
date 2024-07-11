use std::sync::LazyLock;

use device_query::{DeviceQuery, DeviceState, Keycode, MouseButton};

static DEVICE_STATE: LazyLock<DeviceState> = LazyLock::new(|| DeviceState::new());

pub fn is_key_event(key: &Keycode) -> bool {
    let keys: Vec<Keycode> = DEVICE_STATE.get_keys();
    keys.contains(key)
}

pub fn is_mouse_event(mouse_button: &MouseButton) -> bool {
    let mouse_buttons = DEVICE_STATE.get_mouse();
    let buttons_pressed = mouse_buttons.button_pressed;

    buttons_pressed[*mouse_button]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Requires user input"]
    fn test_key_event() {
        while !is_key_event(&Keycode::A) {
            dbg!("Hold 'A' key to continue the test");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        dbg!("Key 'A' pressed");
        std::thread::sleep(std::time::Duration::from_secs(5));

        while !is_key_event(&Keycode::A) {
            dbg!("Hold 'A' key to continue the test");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    #[test]
    #[ignore = "Requires user input"]
    fn test_mouse_event() {
        while !is_mouse_event(&1) {
            dbg!("Hold left mouse button to continue the test");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        dbg!("Left mouse button pressed");
        std::thread::sleep(std::time::Duration::from_secs(5));

        while !is_mouse_event(&1) {
            dbg!("Hold left mouse button to continue the test");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
