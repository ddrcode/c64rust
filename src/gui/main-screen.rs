use cursive::view::Nameable;
use cursive::views::{LinearLayout, PaddedView, ResizedView};
use cursive_tabs::TabPanel;

pub fn mainscreen() -> PaddedView<LinearLayout> {
    // let mut tab_panel = TabPanel::new()
    //     .with_tab(ResizedView::with_full_screen(request_panel::panel()).with_name("Request"))
    //     .with_tab(result_panel::panel().with_name("Response"));
    //
    // tab_panel.set_active_tab("Request").unwrap();
    //
    // PaddedView::lrtb(
    //     3,
    //     3,
    //     1,
    //     1,
    //     LinearLayout::vertical()
    //         .child(ResizedView::with_full_width(url_panel::panel()))
    //         .child(tab_panel.with_name("tabitens")),
    // )
}

