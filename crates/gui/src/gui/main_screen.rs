use super::{cpu_state, get_asm_view, get_variables_view, memory_view, MachineScreen};
use cursive::views::{LinearLayout, NamedView};

/// Creates main screen of the gui and arranges together
/// all major components
pub fn main_screen() -> LinearLayout {
    let mem_and_c64 = LinearLayout::horizontal()
        .child(NamedView::new("machine_screen", MachineScreen::new()))
        .child(memory_view());

    // let debug = ResizedView::with_fixed_size((80, 20), DebugView::new());

    let lines = LinearLayout::vertical()
        .child(mem_and_c64)
        .child(cpu_state(String::new()))
        .child(get_asm_view());

    LinearLayout::horizontal()
        .child(get_variables_view())
        .child(lines)
}
