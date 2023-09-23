use super::UIState;
use cursive::{
    event::Key,
    view::{Nameable, Resizable},
    views::{Dialog, EditView, NamedView, OnEventView, PaddedView, ResizedView},
    Cursive,
};

pub fn address_dialog() -> OnEventView<Dialog> {
    OnEventView::new(
        Dialog::new()
            .title("Enter address")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .on_submit(on_submit)
                    .max_content_width(4)
                    .with_name("address_dialog")
                    .fixed_width(20),
            )
            .button("Ok", |s| {
                let name = s
                    .call_on_name("address_dialog", |view: &mut EditView| view.get_content())
                    .unwrap();
                on_submit(s, &name);
            }),
    )
    .on_event(Key::Esc, |s| {
        s.pop_layer();
    })
}

fn on_submit(s: &mut Cursive, addr_str: &str) {
    match u16::from_str_radix(addr_str, 16) {
        Ok(addr) => {
            s.with_user_data(|data: &mut UIState| {
                data.addr_from = addr;
            });
            s.pop_layer();
        }
        Err(_) => {
            s.add_layer(Dialog::info(format!(
                "'{addr_str}' is not a valid 16-bit hex number"
            )));
        }
    };
}
