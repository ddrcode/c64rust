use crate::{config::CONFIG, messaging::send_client_event};

use super::UIState;
use cursive::{
    event::Key,
    view::{Nameable, Resizable},
    views::{Dialog, EditView, OnEventView},
    Cursive,
};
use machine::client::ClientEvent;

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
            let start_addr = std::cmp::min(addr - addr % 8, 0xffff - CONFIG.memory_view_size);
            send_client_event(ClientEvent::SetObservedMemory(
                start_addr..(start_addr + CONFIG.memory_view_size),
            ));
            s.with_user_data(|data: &mut UIState| {
                data.addr_from = start_addr;
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
