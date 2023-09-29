#![allow(non_camel_case_types)]

use super::UIState;
use crate::utils::keyboard::map_key_event;
use c64::key_utils::screen_code_to_ascii;
use cursive::{
    event::{Callback, Event, EventResult},
    theme::{Color, ColorStyle},
    Printer, Vec2, View,
};
use keyboard_types::Key;

pub struct MachineScreen {
    state: Vec<u8>,
    screen_size: Vec2,
    color: ColorStyle,
    reversed_color: ColorStyle,
    frame_color: ColorStyle,
    character_set: u8,
}

impl MachineScreen {
    pub fn new() -> Self {
        MachineScreen {
            state: [0u8; 40 * 25].to_vec(),
            screen_size: Vec2::new(44, 27),
            reversed_color: ColorStyle::new(
                Color::Rgb(0x35, 0x28, 0x79),
                Color::Rgb(0x70, 0xa4, 0xb2),
            ),
            frame_color: ColorStyle::new(
                Color::Rgb(0x35, 0x28, 0x79),
                Color::Rgb(0x70, 0xa4, 0xb2),
            ),
            color: ColorStyle::new(Color::Rgb(0x70, 0xa4, 0xb2), Color::Rgb(0x35, 0x28, 0x79)),
            character_set: 14,
        }
    }
}

impl MachineScreen {
    pub fn set_state(&mut self, state: Vec<u8>, char_set: u8) {
        if self.state != state {
            self.state = state;
        }
        self.character_set = char_set;
    }

    // draw chars for charcode > 127 (reversed colors)
    fn draw_reversed(&self, printer: &Printer) {
        let cs = self.character_set;
        printer.with_color(self.reversed_color, |p| {
            self.state
                .iter()
                .enumerate()
                .filter(|(_, c)| *c > &127u8)
                .for_each(|(i, c)| {
                    p.print(pos(i), &screen_code_to_ascii(c, cs).to_string());
                });
        });
    }

    fn draw_frame(&self, printer: &Printer) {
        printer.with_color(self.frame_color, |printer| {
            for i in 0..27 {
                printer.print((0, i), &" ".repeat(44));
            }
        });
    }

    fn draw_content(&self, printer: &Printer) {
        printer.with_color(self.color, |printer| {
            for (i, chunk) in self.get_chars().chunks(40).enumerate() {
                printer.print((0, i), &String::from_iter(chunk));
            }
        });
    }

    fn get_chars(&self) -> Vec<char> {
        self.state
            .iter()
            .map(|c| screen_code_to_ascii(c, self.character_set))
            .collect()
    }
}

impl View for MachineScreen {
    fn draw(&self, printer: &Printer) {
        let screen_printer = printer.offset(Vec2::new(2, 1));
        self.draw_frame(printer);
        self.draw_content(&screen_printer);
        self.draw_reversed(&screen_printer);
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        self.screen_size
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        log::warn!("mamy event {:?}", event);
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

fn pos(i: usize) -> (usize, usize) {
    (i % 40, i / 40)
}
