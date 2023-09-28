use cursive::event::{Event, Key as CKey};
use keyboard_types::{Code, Key, KeyState, KeyboardEvent, Modifiers};
use std::collections::HashMap;
use std::str::FromStr;

lazy_static! {
    static ref C64_PUNCTATION_KEYS: Vec<char> = String::from("+-=:;,./*@ £").chars().collect();
    static ref C64_SHIFTED_PUNCTATION_KEYS: HashMap<char, char> = HashMap::from([
        ('!', '1'),
        ('"', '2'),
        ('#', '3'),
        ('$', '4'),
        ('%', '5'),
        ('&', '6'),
        ('\'', '7'),
        ('(', '8'),
        (')', '9'),
        ('<', ','),
        ('>', '.'),
        ('?', '/'),
        ('[', ':'),
        (']', ';'),
    ]);
    static ref C64_SPECIAL_KEYS: HashMap<CKey, (Key, Code, bool)> = HashMap::from([
        (CKey::Enter, (Key::Enter, Code::Enter, false)),
        (CKey::Backspace, (Key::Backspace, Code::Backspace, false)),
        (CKey::Home, (Key::Home, Code::Home, false)),
        (CKey::Up, (Key::ArrowDown, Code::ArrowDown, true)),
        (CKey::Down, (Key::ArrowDown, Code::ArrowDown, false)),
        (CKey::Right, (Key::ArrowRight, Code::ArrowRight, false)),
        (CKey::Left, (Key::ArrowRight, Code::ArrowRight, true)),
    ]);
}

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

pub fn key_event_from_ckey(ckey: CKey, modifiers: Modifiers) -> KeyboardEvent {
    C64_SPECIAL_KEYS
        .get(&ckey)
        .map_or(KeyboardEvent::default(), |(key, code, shift)| {
            KeyboardEvent {
                key: key.clone(),
                code: *code,
                modifiers: modifiers.union(if *shift {
                    Modifiers::SHIFT
                } else {
                    Modifiers::empty()
                }),
                ..Default::default()
            }
        })
}

pub fn map_key_event(e: Event) -> KeyboardEvent {
    let no_key = KeyboardEvent::default();
    match e {
        Event::Char(c) => {
            if c.is_ascii_lowercase() || c.is_ascii_digit() || C64_PUNCTATION_KEYS.contains(&c) {
                key_event_from_char(c, Modifiers::empty())
            } else if c.is_ascii_uppercase() {
                key_event_from_char(c.to_ascii_lowercase(), Modifiers::SHIFT)
            } else if C64_SHIFTED_PUNCTATION_KEYS.contains_key(&c) {
                key_event_from_char(C64_SHIFTED_PUNCTATION_KEYS[&c], Modifiers::SHIFT)
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
