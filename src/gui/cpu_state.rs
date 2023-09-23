use cursive::{
    event::Key,
    theme::*,
    view::{Nameable, Resizable},
    views::*,
    Cursive,
};

pub struct CpuState {
    pub state: String,
}

pub fn cpu_state(state: String) -> NamedView<Canvas<CpuState>> {
    Canvas::new(CpuState { state: state })
        .with_draw(|state: &CpuState, printer| {
            let color =
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::Dark(BaseColor::Cyan));

            printer.with_color(color, |printer| {
                printer.print(
                    (0, 0),
                    &format!("{: ^89}", format!("CPU STATE --- {} ---", state.state)),
                );
            })
        })
        .with_required_size(|_, _| (89, 1).into())
        .with_name("cpu")
}
