use cursive::views::{LinearLayout, NamedView};

use super::{cpu_state, get_asm_view, memory_view, MachineScreen};
use c64::C64;
use machine::{utils::lock, Machine};
use std::sync::{Arc, Mutex};

pub fn main_screen(c64: Arc<Mutex<C64>>) -> LinearLayout {
    let cpu = {
        let arc = Arc::clone(&c64);
        let c64 = lock(&arc);

        c64.cpu().registers.to_string()
    };

    let line = LinearLayout::horizontal()
        .child(NamedView::new("machine_screen", MachineScreen::new(c64)))
        .child(memory_view());

    // let debug = ResizedView::with_fixed_size((80, 20), DebugView::new());

    LinearLayout::vertical()
        .child(line)
        .child(cpu_state(cpu))
        .child(get_asm_view())
}
