#![allow(non_camel_case_types)]

use crate::c64::{C64KeyCode, C64};
use crate::utils::lock;
use cursive::{
    event::{Event, EventResult, Key},
    theme::{Color, ColorStyle},
    Printer, Vec2, View,
};

use std::sync::{Arc, Mutex};
use substring::Substring;

pub struct MachineScreen {
    screen_size: Vec2,
    c64: Arc<Mutex<C64>>,
}

impl MachineScreen {
    pub fn new(c64: Arc<Mutex<C64>>) -> Self {
        MachineScreen {
            c64: c64,
            screen_size: Vec2::new(44, 27),
        }
    }
}

impl View for MachineScreen {
    fn draw(&self, printer: &Printer) {
        let frame_color =
            ColorStyle::new(Color::Rgb(0x35, 0x28, 0x79), Color::Rgb(0x70, 0xa4, 0xb2));
        let color = ColorStyle::new(Color::Rgb(0x70, 0xa4, 0xb2), Color::Rgb(0x35, 0x28, 0x79));

        printer.with_color(frame_color, |printer| {
            for i in 0..27 {
                printer.print((0, i), &" ".repeat(44));
            }
        });

        let x_padding = 2;
        let y_padding = 1;
        let screen_padding = cursive::Vec2::new(x_padding, y_padding);
        let txt = lock(&self.c64).get_screen_memory();
        let screen_printer = printer.offset(screen_padding);
        screen_printer.with_color(color, |printer| {
            for i in 0..25 {
                printer.print((0, i), &format!("{}", txt.substring(i * 40, (i + 1) * 40)));
            }
        });
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        self.screen_size
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char(ch) => {
                if !ch.is_ascii() {
                    return EventResult::Ignored;
                };
                let mut c64 = lock(&self.c64);
                if ch.is_ascii_uppercase() {
                    c64.send_key_with_modifier(
                        C64KeyCode::from(ch.to_ascii_lowercase()),
                        C64KeyCode::RShift,
                    );
                } else {
                    if ch=='"' {
                        c64.send_key_with_modifier(
                            C64KeyCode::from('2'),
                            C64KeyCode::RShift,
                        );
                    } else if ch=='$' {
                        c64.send_key_with_modifier(
                            C64KeyCode::from('4'),
                            C64KeyCode::RShift,
                        );
                    } else {
                    c64.send_key(C64KeyCode::from(ch));}
                }
                EventResult::Consumed(None)
            }
            Event::Key(key) => {
                use C64KeyCode::*;
                let kc = match key {
                    Key::Enter => Return,
                    Key::Backspace => Delete,
                    _ => return EventResult::Ignored,
                };
                lock(&self.c64).send_key(kc);
                EventResult::Consumed(None)
            }
            _ => EventResult::Ignored,
        }
    }
}
