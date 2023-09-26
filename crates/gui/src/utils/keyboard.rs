use std::str::FromStr;
use cursive::event::{Event, EventResult, Key as CKey};
use keyboard_types::{Code, Key, KeyState, KeyboardEvent, Modifiers};

pub fn char_to_code(c: char) -> Code {
    let code = if c.is_ascii_alphabetic() {
        Code::from_str(&["Key", &c.to_ascii_uppercase().to_string()].join(""))
    } else if c.is_ascii_digit() {
        Code::from_str(&["Digit", &c.to_string()].join(""))
    } else {
        log::warn!("Couldn't convert key '{c}' into Code");
        Ok(Code::default())
    };

    code.unwrap_or_else(|e| {
        log::error!("Error converting '{c}' into Code: {e}");
        Code::default()
    })
}

pub fn key_event_from_char(c: char, modifiers: Modifiers) -> KeyboardEvent {
    KeyboardEvent {
        state: KeyState::Down,
        key: Key::Character(c.to_string()),
        code: char_to_code(c),
        modifiers,
        ..Default::default()
    }
}

pub fn ckey_into_key(key: CKey) -> (Key, Code) {
    match key {
        CKey::Enter => (Key::Enter, Code::Enter),
        CKey::Backspace => (Key::Backspace, Code::Backspace),
        _ => {
            log::warn!("Can't convert Cursive key: {:?}", key);
            (Key::Unidentified, Code::Unidentified)
        }
    }
}

pub fn key_event_from_ckey(ckey: CKey, modifiers: Modifiers) -> KeyboardEvent {
    let (key, code) = ckey_into_key(ckey);
    KeyboardEvent {
        key,
        code,
        modifiers,
        ..Default::default()
    }
}

pub fn map_key_event(e: Event) -> KeyboardEvent {
    let no_key = KeyboardEvent::default();
    match e {
        Event::Char(c) => {
            if c.is_ascii_lowercase() || c.is_ascii_digit() {
                key_event_from_char(c, Modifiers::empty())
            } else if c.is_ascii_uppercase() {
                key_event_from_char(c, Modifiers::SHIFT)
            } else {
                no_key
            }
        }

        Event::Key(ckey) => key_event_from_ckey(ckey, Modifiers::empty()),
        Event::Shift(ckey) => key_event_from_ckey(ckey, Modifiers::SHIFT),
        Event::Ctrl(ckey) => key_event_from_ckey(ckey, Modifiers::CONTROL),
        Event::Alt(ckey) => key_event_from_ckey(ckey, Modifiers::ALT),
        Event::CtrlShift(ckey) => key_event_from_ckey(ckey, Modifiers::SHIFT | Modifiers::CONTROL),

        _ => no_key,
    }
}
