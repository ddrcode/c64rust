use cursive::{
    view::{Nameable, Resizable, Scrollable, SizeConstraint},
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
        .scrollable()
        .wrap_with(|v| {
            let mut rv = ResizedView::with_min_height(12, v);
            rv.set_constraints(SizeConstraint::Free, SizeConstraint::AtMost(16));
            rv
        })
        .wrap_with(Panel::new)
        .title("Variables")
}
