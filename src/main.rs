#![no_std]
#![no_main]
#![deny(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::similar_names)]

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
const LINE_HEIGHT: i32 = 12;
/// How many apps fit on the same page
const PER_SCREEN: usize = 12;

enum Command {
    GoDown,
    GoUp,
    Launch,
}

struct App {
    id: String,
    author_id: String,
    name: String,
    author_name: String,
}

/// All the global state. Created in [`boot`], updated in [`update`] and [`render`].
struct State {
    font: FileBuf,
    /// The list of all installed apps.
    apps: Vec<App>,
    /// The currently selected app index.
    pos: usize,
    /// The index of the firs app on the screen.
    top_pos: usize,
    /// The state of buttons on the previous frame.
    old_buttons: Buttons,
    /// The state of direction buttons on the previous frame.
    old_dpad: DPad,
    /// The next command to run when rendering.
    command: Option<Command>,
    /// For how long up or down button (pad) is held.
    held_for: u32,
    shift: i32,
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
        top_pos: 0,
        old_buttons: Buttons::default(),
        old_dpad: DPad::default(),
        command: None,
        held_for: 0,
        shift: 0,
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
            let meta_path = format!("{app_path}/_meta");
            let meta_raw = if let Some(meta_raw) = sudo::load_file_buf(&meta_path) {
                meta_raw
            } else {
                let meta_path = format!("{app_path}/meta");
                let Some(meta_raw) = sudo::load_file_buf(&meta_path) else {
                    continue;
                };
                meta_raw
            };
            let Ok(meta) = Meta::decode(meta_raw.data()) else {
                continue;
            };
            // Hide the launcher itself from the list.
            if meta.author_id == "sys" && meta.app_id == "launcher" {
                continue;
            }
            apps.push(App {
                id: meta.app_id.to_string(),
                author_id: meta.author_id.to_string(),
                name: meta.app_name.to_string(),
                author_name: meta.author_name.to_string(),
            });
        }
    }
    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps
}

#[no_mangle]
extern fn update() {
    let state = get_state();
    handle_input(state);
    apply_command(state);
}

#[no_mangle]
extern fn render() {
    clear_screen(Color::White);
    let state = get_state();
    if state.apps.is_empty() {
        render_empty(state);
        return;
    }
    draw_selection(state);
    draw_apps(state);
    draw_arrows(state);
    draw_scroll(state);
}

/// Render the list of installed apps.
fn draw_apps(state: &State) {
    let font = state.font.as_font();
    let apps = state.apps.iter().skip(state.top_pos).take(PER_SCREEN + 1);
    for (i, app) in apps.enumerate() {
        let point = Point::new(6, 9 + i as i32 * LINE_HEIGHT);
        draw_text(&app.name, &font, point, Color::DarkBlue);
        // Don't show the author name
        // if the app name takes more than half of the screen.
        if app.name.len() > 19 {
            continue;
        }
        // Don't show the author name if it doesn't fit on the screen.
        if app.author_name.len() > 16 {
            continue;
        }
        let point = Point::new(WIDTH / 2 + 6, 9 + i as i32 * LINE_HEIGHT);
        let text = format!("by {}", &app.author_name);
        draw_text(&text, &font, point, Color::LightGray);
    }
}

/// Draw ap and/or down arrows on the right if not all apps fit on the screen.
fn draw_arrows(state: &State) {
    const SCROLL_WIDTH: i32 = 6;
    let style = Style {
        fill_color: Color::DarkBlue,
        ..Style::default()
    };
    if state.top_pos > 0 {
        draw_triangle(
            Point::new(WIDTH - SCROLL_WIDTH - 4, 5),
            Point::new(WIDTH - 4, 5),
            Point::new(WIDTH - SCROLL_WIDTH / 2 - 4, 5 - SCROLL_WIDTH / 2),
            style,
        );
    }
    if state.apps.len() - 1 > state.top_pos + PER_SCREEN {
        draw_triangle(
            Point::new(WIDTH - SCROLL_WIDTH - 4, HEIGHT - 5 - SCROLL_WIDTH / 2),
            Point::new(WIDTH - 4, HEIGHT - 5 - SCROLL_WIDTH / 2),
            Point::new(WIDTH - SCROLL_WIDTH / 2 - 4, HEIGHT - 5),
            style,
        );
    }
}

fn draw_scroll(state: &State) {
    const SCROLL_WIDTH: i32 = 6;
    const SCROLL_HEIGHT: usize = HEIGHT as usize - 4;
    if state.apps.len() - 1 <= PER_SCREEN {
        return;
    }
    draw_rounded_rect(
        Point::new(
            WIDTH - SCROLL_WIDTH - 4,
            (SCROLL_HEIGHT * state.pos / state.apps.len()) as i32 + 2,
        ),
        Size::new(SCROLL_WIDTH + 1, 10),
        Size::new(4, 6),
        Style {
            fill_color: Color::DarkBlue,
            ..Style::default()
        },
    );
}

fn handle_input(state: &mut State) {
    let new_buttons = read_buttons(Player::P0);
    let new_pad = read_pad(Player::P0).unwrap_or_default();
    let new_dpad = new_pad.as_dpad();

    // If a direction button is held, track for how long.
    if new_dpad.up || new_dpad.down {
        state.held_for += 1;
    } else {
        state.held_for = 0;
    }

    let pressed_buttons = new_buttons.just_pressed(&state.old_buttons);
    let command = if pressed_buttons.a {
        Some(Command::Launch)
    } else if state.held_for > 30 && state.held_for % 4 == 0 {
        // a button is held for 0.5s
        Some(if new_dpad.up {
            Command::GoUp
        } else {
            Command::GoDown
        })
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

    // Shift and stutter the selection when the first or the last app
    // is selected to indicate that there are no more items on the list.
    if state.held_for % 30 >= 25 {
        state.shift = 0;
    } else if new_dpad.down && state.pos + 1 == state.apps.len() {
        state.shift = 1;
    } else if new_dpad.up && state.pos == 0 {
        state.shift = -1;
    } else {
        state.shift = 0;
    }

    // If the selection cursor tries to go out of screen,
    // scroll the list to keep the selection on the screen.
    if state.pos > state.top_pos + PER_SCREEN {
        state.top_pos = state.pos - PER_SCREEN;
    } else if state.pos < state.top_pos {
        state.top_pos = state.pos;
    }

    state.old_buttons = new_buttons;
    state.old_dpad = new_dpad;
    state.command = command;
}

fn draw_selection(state: &State) {
    const MARGIN: i32 = 3;
    let pos = state.pos.saturating_sub(state.top_pos);
    let mut width = WIDTH - MARGIN * 2;
    // If not all apps fit on the screen, give some space on the right
    // for the scroll bar.
    if state.apps.len() - 1 > PER_SCREEN {
        width -= 10;
    }
    draw_rounded_rect(
        Point::new(MARGIN, 2 + pos as i32 * LINE_HEIGHT + state.shift),
        Size::new(width, LINE_HEIGHT),
        Size::new(4, 4),
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
                state.pos += 1;
            }
        }
        Command::GoUp => {
            state.pos = state.pos.saturating_sub(1);
        }
        Command::Launch => {
            if let Some(app) = state.apps.get(state.pos) {
                sudo::run_app(&app.author_id, &app.id);
            }
        }
    }
}

/// Show message about no apps (except launcher itself) installed.
fn render_empty(state: &State) {
    let font = state.font.as_font();
    let point = Point::new(62, HEIGHT / 2 - LINE_HEIGHT / 2);
    draw_text("No apps installed", &font, point, Color::DarkBlue);
}
