use super::UIState;
use cursive::{
    event::{EventResult, Key},
    theme::Style,
    traits::{Nameable, With},
    view::{scroll::Scroller, ScrollStrategy, Scrollable, View},
    views::*,
    Cursive,
};

pub fn update_asm_view(s: &mut Cursive, last_op: &String, next_op: &String) {
    let lines = if let Some(ud) = s.user_data::<UIState>() {
        ud.asm_lines.pop();
        if let Some(last) = ud.asm_lines.pop() {
            ud.asm_lines.push(last.to_string().replacen(">", " ", 1));
        }
        ud.asm_lines.push(["> ",last_op].join(""));
        ud.asm_lines.push(["  ",next_op].join(""));
        if ud.asm_lines.len() > 100 {
            ud.asm_lines.remove(0);
        }
        ud.asm_lines.join("\n")
    } else {
        log::warn!("No user data found in update_asm_view");
        String::new()
    };

    s.call_on_name("asm", move |view: &mut TextView| {
        view.set_content(lines);
    });
}

pub fn get_asm_view() -> impl View {
    let style = Style::default();

    TextView::new("")
        .style(style)
        .with_name("asm")
        .scrollable()
        .scroll_strategy(ScrollStrategy::StickToBottom)
        .wrap_with(OnEventView::new)
        .on_pre_event_inner(Key::PageUp, |v, _| {
            let scroller = v.get_scroller_mut();
            if scroller.can_scroll_up() {
                scroller.scroll_up(scroller.last_outer_size().y.saturating_sub(1));
            }
            Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner(Key::PageDown, |v, _| {
            let scroller = v.get_scroller_mut();
            if scroller.can_scroll_down() {
                scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
            }
            Some(EventResult::Consumed(None))
        })
        .wrap_with(|v| PaddedView::lrtb(0, 0, 0, 0, v))
        .wrap_with(|v| ResizedView::with_fixed_height(10, v))
        .wrap_with(|v| {
            let mut hv = HideableView::new(v);
            hv.hide();
            hv
        })
        .with_name("asm_wrapper")
}
