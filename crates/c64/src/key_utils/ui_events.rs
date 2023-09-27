use crate::c64::C64KeyCode;
use keyboard_types::{Key, KeyState, KeyboardEvent, Modifiers};

pub fn ui_event_to_c64_key_codes(ke: &KeyboardEvent) -> Vec<C64KeyCode> {
    let mut keys = Vec::new();

    if ke.modifiers.shift() {
        keys.push(C64KeyCode::LShift)
    };
    if ke.modifiers.ctrl() {
        keys.push(C64KeyCode::Ctrl)
    };
    if ke.modifiers.alt() {
        keys.push(C64KeyCode::Cmd)
    }; // LAlt maps to Commodore key

    match &ke.key {
        Key::Character(ch) => ch
            .chars()
            .map(|c: char| C64KeyCode::from_char(c))
            .filter(|x| x.is_some())
            .for_each(|key| keys.push(key.unwrap())),
        Key::Enter => keys.push(C64KeyCode::Return),
        Key::Backspace => keys.push(C64KeyCode::Delete),
        Key::ArrowRight => keys.push(C64KeyCode::CursorLR),
        Key::ArrowDown => keys.push(C64KeyCode::CursorUD),
        Key::Home => keys.push(C64KeyCode::Home),
        _ => {
            log::warn!("Sending unrecognized key to C64: {}", &ke.key);
            ()
        }
    };

    if ke.state == KeyState::Up {
        keys.reverse()
    }

    keys
}
