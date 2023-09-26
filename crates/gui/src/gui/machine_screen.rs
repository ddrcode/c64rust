#![allow(non_camel_case_types)]

use crate::utils::keyboard::map_key_event;
use cursive::{
    event::{Callback, Event, EventResult},
    theme::{Color, ColorStyle},
    Printer, Vec2, View,
};
use keyboard_types::Key;
use substring::Substring;

use super::UIState;

pub struct MachineScreen {
    state: String,
    screen_size: Vec2,
}

impl MachineScreen {
    pub fn new() -> Self {
        MachineScreen {
            state: String::from(" ".repeat(40 * 25)),
            screen_size: Vec2::new(44, 27),
        }
    }
}

impl MachineScreen {
    pub fn set_state(&mut self, state: String) {
        if self.state != state {
            self.state = state;
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
        let screen_printer = printer.offset(screen_padding);
        screen_printer.with_color(color, |printer| {
            for i in 0..25 {
                printer.print(
                    (0, i),
                    &format!("{}", self.state.substring(i * 40, (i + 1) * 40)),
                );
            }
        });
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        self.screen_size
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        let event = map_key_event(event);
        if event.key == Key::Unidentified {
            EventResult::Ignored
        } else {
            EventResult::Consumed(Some(Callback::from_fn_once(|s| {
                s.with_user_data(|data: &mut UIState| {
                    data.key = Some(event);
                });
            })))
        }
    }
}
