use cursive::views::{LinearLayout, NamedView, PaddedView, ResizedView};
// use cursive_tabs::TabPanel;

use super::{cpu_state, Screen};
use crate::c64::C64;
use crate::machine::Machine;
use crate::utils::lock;
use cursive_hexview::{DisplayState, HexView, HexViewConfig};
use std::sync::{Arc, Mutex};

pub fn main_screen(c64: Arc<Mutex<C64>>) -> LinearLayout {
    let config = HexViewConfig {
        bytes_per_line: 8,
        bytes_per_group: 1,
        byte_group_separator: " ",
        show_ascii: true,
        hex_ascii_separator: "   ",
        bytes_per_addr: 4,
        ..Default::default()
    };

    let (memory, cpu) = {
        let arc = Arc::clone(&c64);
        let c64 = lock(&arc);
        (
            c64.memory().fragment(0, 200),
            c64.cpu().registers.to_string(),
        )
    };

    let mut hex_view = HexView::new_from_iter(memory.iter()).display_state(DisplayState::Disabled);
    hex_view.set_config(config);

    let hex_pane = ResizedView::with_fixed_size(
        (45, 27),
        PaddedView::lrtb(2, 2, 1, 1, NamedView::new("memory", hex_view)),
    );

    let line = LinearLayout::horizontal()
        .child(Screen::new(c64))
        .child(hex_pane);

    // let debug = ResizedView::with_fixed_size((80, 20), DebugView::new());

    LinearLayout::vertical().child(line).child(cpu_state(cpu))
}
