use cursive::theme::Color;

pub fn color(rgb: &str) -> Color {
    Color::parse(rgb).unwrap()
}
