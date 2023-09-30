use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use c64::{C64Client, MachineState};
use cursive::{
    event::{Event, Key},
    menu,
    view::ViewWrapper,
    views::*,
    Cursive, CursiveRunnable,
};
use cursive_hexview::HexView;
use machine::MachineStatus;
use machine::{client::NonInteractiveClient, utils::lock, MachineError};

use crate::gui::{address_dialog, main_screen, UIState};

use super::{update_asm_view, update_variables_view, CpuState, MachineScreen};

static FIRST_DEBUG: AtomicBool = AtomicBool::new(true);

type AsmIsEasierThanThis = ResizedView<PaddedView<OnEventView<ScrollView<NamedView<TextView>>>>>;
type OnceUponAMidnightDreary = PaddedView<LinearLayout>;

pub(crate) fn update_ui(state: &MachineState, s: &mut Cursive) {
    let addr = s.user_data::<UIState>().map_or(0, |data| data.addr_from);
    let screen = state.screen.clone();

    s.call_on_name("memory", |view: &mut HexView| {
        view.config_mut().start_addr = addr as usize;
        view.set_data(state.memory_slice.iter());
    });

    s.call_on_name("cpu", |view: &mut Canvas<CpuState>| {
        view.state_mut().state = state.registers.to_string();
    });

    s.call_on_name("machine_screen", move |view: &mut MachineScreen| {
        view.set_state(screen, state.character_set);
    });

    update_asm_view(s, &state.last_op, &state.next_op);
    update_variables_view(s, &state.debugger.variables);

    if FIRST_DEBUG.load(Ordering::Relaxed) && state.status == MachineStatus::Debug {
        FIRST_DEBUG.store(false, Ordering::Relaxed);
        set_visible::<AsmIsEasierThanThis>(s, "asm_wrapper", true);
        set_visible::<OnceUponAMidnightDreary>(s, "variables_panel", true);
    }
}

pub(crate) fn init_ui(client: Arc<Mutex<C64Client>>) -> CursiveRunnable {
    let mut siv = cursive::default();
    set_theme(&mut siv);
    siv.set_autorefresh(false);
    siv.set_autohide_menu(false);

    let quit_handler = {
        let arc = client.clone();
        move |s: &mut Cursive| {
            s.quit();
            lock(&arc).stop().unwrap_or_else(handle_error);
        }
    };

    let reset_handler = {
        let arc = client.clone();
        move |_s: &mut Cursive| {
            lock(&arc).reset().unwrap_or_else(handle_error);
        }
    };

    let debug_handler = {
        use machine::MachineStatus::*;
        let arc = Arc::clone(&client);
        move |s: &mut Cursive| {
            let mut c64 = lock(&arc);
            (match c64.get_status() {
                Running => {
                    set_visible::<AsmIsEasierThanThis>(s, "asm_wrapper", true);
                    set_visible::<OnceUponAMidnightDreary>(s, "variables_panel", true);
                    c64.pause()
                }
                Debug => c64.resume(),
                _ => Ok(()),
            })
            .unwrap_or_else(handle_error);
        }
    };

    let next_handler = {
        let arc = client.clone();
        move |_s: &mut Cursive| {
            lock(&arc).next().unwrap_or_else(|err| {
                handle_error(err);
                false
            });
        }
    };

    let screen = main_screen();

    siv.menubar()
        .add_subtree(
            "Machine",
            menu::Tree::new()
                .leaf("Restart", reset_handler)
                .leaf("Stop (and quit)", quit_handler.clone()),
        )
        .add_subtree(
            "Debug",
            menu::Tree::new()
                .leaf("Go to address [F6]", |s| s.add_layer(address_dialog()))
                .leaf("Toggle debugging [F7]", debug_handler.clone())
                .leaf("Next step [F8]", next_handler.clone())
                .leaf("Skip interrupts", |_s| {}),
        )
        .add_subtree(
            "View",
            menu::Tree::new()
                .leaf("Toggle memory view", |_s| ())
                .leaf("Toggle processor status", |_s| ())
                .leaf(
                    "Toggle disassembly view [F2]",
                    create_toggle_handler::<AsmIsEasierThanThis>("asm_wrapper"),
                )
                .leaf(
                    "Toggle variables/breakpoints view [F3]",
                    create_toggle_handler::<OnceUponAMidnightDreary>("variables_panel"),
                ),
        )
        .add_leaf("Quit", quit_handler.clone());

    siv.add_global_callback(Key::F9, |s| s.select_menubar());
    siv.add_global_callback(Key::F10, quit_handler);
    siv.add_global_callback(Key::F6, |s| s.add_layer(address_dialog()));
    siv.add_global_callback(Key::F7, debug_handler);
    siv.add_global_callback(Key::F8, next_handler);
    siv.add_global_callback(Event::Char('`'), cursive::Cursive::toggle_debug_console);
    siv.add_global_callback(
        Key::F3,
        create_toggle_handler::<AsmIsEasierThanThis>("asm_wrapper"),
    );
    siv.add_global_callback(
        Key::F2,
        create_toggle_handler::<OnceUponAMidnightDreary>("variables_panel"),
    );

    siv.add_layer(screen);
    siv.set_user_data(UIState::new());

    siv.focus_name("machine_screen").unwrap();
    siv
}

fn set_theme(siv: &mut CursiveRunnable) {
    use cursive::theme::*;
    let mut theme = siv.current_theme().clone();
    theme.shadow = true;
    theme.borders = BorderStyle::None;
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::White);
    theme.palette[PaletteColor::View] = Color::Rgb(0x9c, 0xa5, 0xb5);
    // theme.palette[cursive::theme::PaletteColor::View] = cursive::theme::Color::TerminalDefault;
    siv.set_theme(theme);
}

fn create_toggle_handler<V: ViewWrapper>(name: &str) -> impl Fn(&mut Cursive) + '_ {
    |s| {
        s.call_on_name(name, |view: &mut HideableView<V>| {
            let visible = view.is_visible();
            view.set_visible(!visible);
        });
    }
}

fn set_visible<V: ViewWrapper>(s: &mut Cursive, name: &str, visible: bool) {
    s.call_on_name(name, |view: &mut HideableView<V>| {
        view.set_visible(visible);
    });
}

fn handle_error(err: MachineError) {
    log::error!("An error occured on emulator side: {}", err);
    Dialog::info("The emulator has failed!");
}
