use cursive::{
    event::{Callback, Event, EventResult, Key},
    theme::{BaseColor, Color, ColorStyle},
    views::Dialog,
    Printer, Vec2, View,
};

use substring::Substring;

pub struct Screen {
    screen_size: Vec2,
    pub content: String
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            screen_size: Vec2::new(40, 25),
            content: String::from("co jest grane jjjhhheeekkkeeeeeeeeekwkwkwkwjwkwkwkwkw\nkw jk kjkjw jkj kwjk jwkj ")
        }
    }
}

impl View for Screen {
    fn draw(&self, printer: &Printer) {
        let x_padding = 1;
        let y_padding = 1;
        let screen_padding = cursive::Vec2::new(x_padding, y_padding);
        // let board_printer = printer.offset(board_padding);
        for i in 0..25 {
            printer.print((0,i), &format!(" {}", self.content.substring(i*40, (i+1)*40)));
        }
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(44, 29)
    }

    fn on_event(&mut self, event: cursive::event::Event) -> cursive::event::EventResult {
        match event {
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
