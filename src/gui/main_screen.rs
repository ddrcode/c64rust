use cursive::views::{DummyView, LinearLayout, NamedView, PaddedView, ResizedView};
// use cursive_tabs::TabPanel;

use super::Screen;
use crate::c64::C64;
use crate::machine::Machine;
use cursive_hexview::{DisplayState, HexView, HexViewConfig};
use std::sync::{Arc, Mutex};

pub fn main_screen(c64: Arc<Mutex<C64>>) -> LinearLayout {
    log::info!("Starting!");

    let iter = Arc::clone(&c64);
    let c64_screen = Screen::new(c64);

    let config = HexViewConfig {
        bytes_per_line: 8,
        bytes_per_group: 1,
        byte_group_separator: " ",
        show_ascii: true,
        hex_ascii_separator: "   ",
        ..Default::default()
    };

    let mut hex_view = HexView::new_from_iter(iter.lock().unwrap().memory().mem(0).iter())
        .display_state(DisplayState::Disabled);
    hex_view.set_config(config);

    let hex_pane = ResizedView::with_fixed_size(
        (45, 27),
        PaddedView::lrtb(2, 2, 1, 1, NamedView::new("memory", hex_view)),
    );

    let line = LinearLayout::horizontal()
        .child(c64_screen)
        .child(DummyView)
        .child(hex_pane);

    // let debug = ResizedView::with_fixed_size((80, 20), DebugView::new());

    LinearLayout::vertical().child(line)
}

// pub fn mainscreen() -> PaddedView<LinearLayout> {
//     let mut tab_panel = TabPanel::new()
//         .with_tab(ResizedView::with_full_screen(request_panel::panel()).with_name("Request"))
//         .with_tab(result_panel::panel().with_name("Response"));
//
//     tab_panel.set_active_tab("Request").unwrap();
//
//     PaddedView::lrtb(
//         3,
//         3,
//         1,
//         1,
//         LinearLayout::vertical()
//             .child(ResizedView::with_full_width(url_panel::panel()))
//             .child(tab_panel.with_name("tabitens")),
//     )
// }
