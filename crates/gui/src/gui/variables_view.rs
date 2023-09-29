use cursive::{
    view::{Nameable, Scrollable},
    views::{HideableView, PaddedView, Panel, ResizedView, TextView},
    Cursive, View, With,
};
use machine::debugger::Variable;

pub fn update_variables_view(s: &mut Cursive, vars: &Vec<Variable>) {
    let content = vars
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    s.call_on_name("variables", move |view: &mut TextView| {
        view.set_content(content)
    });
}

pub fn get_variables_view() -> impl View {
    TextView::new("")
        .with_name("variables")
        .wrap_with(|v| PaddedView::lrtb(1, 1, 1, 1, v))
        .scrollable()
        .wrap_with(|v| ResizedView::with_min_height(20, v))
        .wrap_with(Panel::new)
        .title("Variables")
        .wrap_with(HideableView::new)
        .hidden()
        .with_name("variables_wrapper")
}
