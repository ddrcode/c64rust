use super::{
    cpu_state, get_asm_view, get_breakpoints_view, get_variables_view, memory_view, MachineScreen,
};
use cursive::{
    view::Nameable,
    views::{HideableView, LinearLayout, NamedView, PaddedView},
    With,
};

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
        .child(
            HideableView::new(
                (LinearLayout::vertical()
                    .child(get_variables_view())
                    .child(get_breakpoints_view()))
                .wrap_with(|v| PaddedView::lrtb(0, 0, 1, 1, v)),
            )
            .hidden()
            .with_name("variables_panel"),
        )
        .child(lines)
}
