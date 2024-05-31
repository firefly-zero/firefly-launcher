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
pub const LINE_HEIGHT: i32 = 12;

enum Command {
    GoDown,
    GoUp,
    Launch,
}

struct App {
    app_name: String,
    author_id: String,
    app_id: String,
}

/// All the global state. Created in [`boot`], updated in [`update`] and [`render`].
struct State {
    font: FileBuf,
    /// The list of all installed apps.
    apps: Vec<App>,
    /// The currently selected app index.
    pos: usize,
    /// The state of buttons on the previous frame.
    old_buttons: Buttons,
    /// The state of direction buttons on the previous frame.
    old_dpad: DPad,
    /// The next command to run when rendering.
    command: Option<Command>,
    /// For how long up or down button (pad) is held.
    held_for: u32,
}

/// Get the global state
fn get_state() -> &'static mut State {
    unsafe { STATE.get_mut() }.unwrap()
}

#[no_mangle]
extern fn boot() {
    let state = State {
        font: rom::load_buf("font"),
        apps: read_apps(),
        pos: 0,
        old_buttons: Default::default(),
        old_dpad: Default::default(),
        command: None,
        held_for: 0,
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
            // Hide the launcher itself from the list.
            if meta.author_id == "sys" && meta.app_id == "launcher" {
                continue;
            }
            apps.push(App {
                app_name: meta.app_name.to_string(),
                author_id: meta.author_id.to_string(),
                app_id: meta.app_id.to_string(),
            });
        }
    }
    apps
}

#[no_mangle]
extern fn update() {
    handle_input();
}

#[no_mangle]
extern fn render() {
    clear_screen(Color::White);
    let state = get_state();
    if (&state.apps).is_empty() {
        render_empty(state);
        return;
    }
    apply_command(state);
    draw_selection(state);
    let font = state.font.as_font();
    for (i, app) in state.apps.iter().enumerate() {
        draw_text(
            &app.app_name,
            &font,
            Point {
                x: 10,
                y: 10 + i as i32 * LINE_HEIGHT,
            },
            Color::DarkBlue,
        );
    }
}

fn handle_input() {
    let state = get_state();
    let new_buttons = read_buttons(Player::P0);
    let new_pad = read_pad(Player::P0).unwrap_or_default();
    let new_dpad = new_pad.as_dpad();

    if new_dpad.up || new_dpad.down {
        // A direction button is held. Track for how long.
        state.held_for += 1
    } else {
        // No direction button is held.
        state.held_for = 0
    }

    let pressed_buttons = new_buttons.just_pressed(&state.old_buttons);
    let command = if pressed_buttons.a {
        Some(Command::Launch)
    } else {
        let pressed_dpad = new_dpad.just_pressed(&state.old_dpad);
        if state.held_for > 30 && state.held_for % 4 == 0 {
            // a button is held for 0.5s
            Some(if new_dpad.up {
                Command::GoUp
            } else {
                Command::GoDown
            })
        } else if pressed_dpad.up {
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
    const MARGIN: i32 = 5;
    draw_rounded_rect(
        Point {
            x: MARGIN,
            y: 3 + state.pos as i32 * LINE_HEIGHT,
        },
        Size {
            width: WIDTH - MARGIN * 2,
            height: LINE_HEIGHT,
        },
        Size {
            width: 4,
            height: 4,
        },
        Style {
            stroke_color: Color::DarkBlue,
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

/// Show message about no apps (except launcher itself) installed.
fn render_empty(state: &mut State) {
    let font = state.font.as_font();
    let point = Point {
        x: 62,
        y: HEIGHT / 2 - LINE_HEIGHT / 2,
    };
    draw_text("No apps installed", &font, point, Color::DarkBlue);
}
