#![no_std]
#![no_main]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_meta::Meta;
use firefly_rust::*;
use talc::locking::AssumeUnlockable;
use talc::{ClaimOnOom, Span, Talc, Talck};

// one wasm page
static mut ARENA: [u8; 65536] = [0; 65536];

// Setup Talc as the global allocator.
#[global_allocator]
static ALLOCATOR: Talck<AssumeUnlockable, ClaimOnOom> = Talc::new(unsafe {
    //
    ClaimOnOom::new(Span::from_const_array(core::ptr::addr_of!(ARENA)))
})
.lock();

static mut STATE: OnceCell<State> = OnceCell::new();

pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 160;

enum Command {
    GoDown,
    GoUp,
    Launch,
}

struct App {
    app_name:  String,
    author_id: String,
    app_id:    String,
}

/// All the global state. Create in [boot], updated in [update] and [render].
struct State {
    font:        FileBuf,
    apps:        Vec<App>,
    pos:         usize,
    old_buttons: Buttons,
    old_dpad:    DPad,
    command:     Option<Command>,
}

/// Get the global state
fn get_state() -> &'static mut State {
    unsafe { STATE.get_mut() }.unwrap()
}

#[no_mangle]
extern fn boot() {
    let state = State {
        font:        rom::load_buf("font"),
        apps:        read_apps(),
        pos:         0,
        old_buttons: Default::default(),
        old_dpad:    Default::default(),
        command:     None,
    };
    unsafe { STATE.set(state) }.ok().unwrap();
}

/// Go through all ROMs and read their metadata.
fn read_apps() -> Vec<App> {
    let author_dirs = sudo::DirBuf::list_dirs("roms");
    let mut apps: Vec<App> = Vec::new();
    for author_dir in author_dirs.iter() {
        let author_path = format!("roms/{author_dir}");
        let app_dirs = sudo::DirBuf::list_dirs(&author_path);
        for app_dir in app_dirs.iter() {
            let app_path = format!("{author_path}/{app_dir}");
            let meta_path = format!("{app_path}/meta");
            let meta_raw = sudo::load_file_buf(&meta_path);
            let Ok(meta) = Meta::decode(meta_raw.data()) else {
                continue;
            };
            apps.push(App {
                app_name:  meta.app_name.to_string(),
                author_id: meta.author_id.to_string(),
                app_id:    meta.app_id.to_string(),
            });
        }
    }
    apps
}

#[no_mangle]
extern fn update() {
    handle_input()
}

#[no_mangle]
extern fn render() {
    let state = get_state();
    clear_screen(Color::Light);
    apply_command(state);
    draw_selection(state);
    let font = state.font.as_font();
    for (i, app) in state.apps.iter().enumerate() {
        draw_text(
            &app.app_name,
            &font,
            Point {
                x: 20,
                y: 10 + i as i32 * 10,
            },
            Color::Dark,
        );
    }
}

fn handle_input() {
    let state = get_state();
    let new_buttons = read_buttons(Player::P0);
    let new_pad = read_pad(Player::P0).unwrap_or_default();
    let new_dpad = new_pad.as_dpad();

    let pressed_buttons = new_buttons.just_pressed(&state.old_buttons);
    let command = if pressed_buttons.a {
        Some(Command::Launch)
    } else {
        let pressed_dpad = new_dpad.just_pressed(&state.old_dpad);
        if pressed_dpad.up {
            Some(Command::GoUp)
        } else if pressed_dpad.down {
            Some(Command::GoDown)
        } else {
            None
        }
    };
    state.old_buttons = new_buttons;
    state.old_dpad = new_dpad;
    state.command = command;
}

fn draw_selection(state: &mut State) {
    draw_circle(
        Point {
            x: 10,
            y: 4 + state.pos as i32 * 10,
        },
        8,
        Style {
            fill_color: Color::Dark,
            ..Style::default()
        },
    );
}

fn apply_command(state: &mut State) {
    let Some(command) = &state.command else {
        return;
    };
    match command {
        Command::GoDown => {
            let apps_count = state.apps.len();
            if state.pos + 1 < apps_count {
                state.pos += 1
            }
        }
        Command::GoUp => {
            state.pos = state.pos.saturating_sub(1);
        }
        Command::Launch => {
            if let Some(app) = state.apps.get(state.pos) {
                sudo::run_app(&app.author_id, &app.app_id);
            }
        }
    }
}
