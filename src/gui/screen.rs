use crate::c64::C64;
use cursive::{
    event::{Callback, Event, EventResult, Key},
    theme::{BaseColor, Color, ColorStyle, BaseColor::*, Color::*, PaletteColor::* },
    views::Dialog,
    Printer, Vec2, View,
};

use std::sync::{Arc, Mutex};
use substring::Substring;

pub struct Screen {
    screen_size: Vec2,
    c64: Arc<Mutex<C64>>,
}

impl Screen {
    pub fn new(c64: Arc<Mutex<C64>>) -> Self {
        Screen {
            c64: c64,
            screen_size: Vec2::new(44, 27),
        }
    }

    fn get_theme() -> cursive::theme::Theme {
        let mut palette = cursive::theme::Palette::default();
        palette[Background] = Dark(Blue);
        palette[Shadow] = Dark(Black);
        palette[View] = Dark(White);
        palette[Primary] = Dark(Black);
        palette[Secondary] = Dark(Blue);
        palette[Tertiary] = Light(White);
        palette[TitlePrimary] = Dark(Red);
        palette[TitleSecondary] = Dark(Yellow);
        palette[Highlight] = Dark(Red);
        palette[HighlightInactive] = Dark(Blue);
        palette[HighlightText] = Dark(White);
        

        palette[Background] = Color::Rgb(0x6c, 0x5e, 0xb5);
        palette[View] = Color::Rgb(0x6c, 0x5e, 0xb5);
        palette[Tertiary] = Color::Rgb(0x6c, 0x5e, 0xb5);
        palette[Shadow] = Color::Rgb(0x6c, 0x5e, 0xb5);
        
        cursive::theme::Theme {
            shadow: true,
            borders: cursive::theme::BorderStyle::Simple,
            palette: palette,
        }
    }
}

impl View for Screen {
    fn draw(&self, printer: &Printer) {
        let color = ColorStyle::new(Color::Rgb(0x6c, 0x5e, 0xb5), Color::Rgb(0x35, 0x28, 0x79));
        let x_padding = 2;
        let y_padding = 1;
        let screen_padding = cursive::Vec2::new(x_padding, y_padding);
        let txt = self.c64.lock().unwrap().get_screen_memory();
        let screen_printer = printer.offset(screen_padding);
        screen_printer.with_color(color, |printer| {
            for i in 0..25 {
                printer.print((0, i), &format!("{}", txt.substring(i * 40, (i + 1) * 40)));
            }
        });
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(44, 27)
    }

    fn on_event(&mut self, event: cursive::event::Event) -> cursive::event::EventResult {
        match event {
            Event::Char(ch) => {
                self.c64.lock().unwrap().send_key(ch);
                EventResult::Consumed(None)
            }
            Event::Key(Key::Enter) => {
                self.c64.lock().unwrap().send_key(char::from(13));
                EventResult::Consumed(None)
            }
            // Event::Char('l') | Event::Key(Key::Left) => self.push(LRUD::Left),
            // Event::Char('r') | Event::Key(Key::Right) => self.push(LRUD::Right),
            // Event::Char('u') | Event::Key(Key::Up) => self.push(LRUD::Up),
            // Event::Char('d') | Event::Key(Key::Down) => self.push(LRUD::Down),
            // Event::Char('n') => {
            //     self.new_game();
            //     EventResult::Consumed(None)
            // }
            _ => EventResult::Ignored,
        }
    }
}
