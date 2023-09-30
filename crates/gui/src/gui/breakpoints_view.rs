use cursive::{
    view::{Nameable, Scrollable, SizeConstraint},
    views::{Checkbox, LinearLayout, Panel, ResizedView, TextView},
    Cursive, View, With,
};
use machine::{client::ClientEvent, debugger::Breakpoint};

use crate::messaging::send_client_event;



pub fn init_breakpoints_view(s: &mut Cursive, bps: &Vec<Breakpoint>) {
    let content: Vec<Breakpoint> = bps
        .iter()
        .filter(|b| {
            if let Breakpoint::Address(_s) = b {
                true
            } else {
                false
            }
        })
        .map(|b| b.clone())
        .collect();

    s.call_on_name("breakpoints", move |view: &mut LinearLayout| {
        if view.is_empty() {
            content.iter().for_each(|bp| {
                if let Breakpoint::Address(addr) = bp {
                    view.add_child(get_checkbox(*addr));
                }
            })
        }
    });
}

fn get_checkbox(addr: u16) -> impl View {
    LinearLayout::horizontal()
        .child(Checkbox::new().checked().on_change(move |_c, value| {
            send_client_event(if value {
                ClientEvent::EnableBreakpoint(Breakpoint::Address(addr))
            } else {
                ClientEvent::DisableBreakpoint(Breakpoint::Address(addr))
            });
        }))
        .child(TextView::new(" "))
        .child(TextView::new(&format!("{:04x}", addr)))
}

pub fn get_breakpoints_view() -> impl View {
    // TextView::new("teste test")
    LinearLayout::vertical()
        .with_name("breakpoints")
        .scrollable()
        .wrap_with(|v| {
            let mut rv = ResizedView::with_min_height(12, v);
            rv.set_constraints(SizeConstraint::Free, SizeConstraint::AtMost(16));
            rv
        })
        .wrap_with(Panel::new)
        .title("Breakpoints")
}
