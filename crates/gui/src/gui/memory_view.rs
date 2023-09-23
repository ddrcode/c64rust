use cursive::{
    view::View,
    views::{NamedView, PaddedView, ResizedView},
};
use cursive_hexview::{DisplayState, HexView, HexViewConfig};

pub fn memory_view() -> impl View {
    let config = HexViewConfig {
        bytes_per_line: 8,
        bytes_per_group: 1,
        byte_group_separator: " ",
        show_ascii: true,
        hex_ascii_separator: "   ",
        bytes_per_addr: 4,
        ..Default::default()
    };

    let mut hex_view = HexView::default().display_state(DisplayState::Disabled);
    hex_view.set_config(config);

    let hex_pane = ResizedView::with_fixed_size(
        (45, 27),
        PaddedView::lrtb(2, 2, 1, 1, NamedView::new("memory", hex_view)),
    );

    hex_pane
}
