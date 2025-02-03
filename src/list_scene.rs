use crate::*;
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;
/// How many apps fit on the same page
const PER_SCREEN: usize = 12;

pub enum Command {
    GoDown,
    GoUp,
    Launch,
}

pub fn init(state: &mut State) {
    state.apps = read_apps();
}

pub fn update(state: &mut State) {
    handle_input(state);
    apply_command(state);
}

pub fn render(state: &State) {
    if let Some(splash_path) = &state.splash {
        let splash = sudo::load_file_buf(splash_path);
        if let Some(splash) = splash {
            draw_image(&splash.as_image(), Point::MIN);
        }
        return;
    }
    clear_screen(Color::White);
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
        draw_text(&app.author_name, &font, point, Color::LightGray);
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
    let new_buttons = read_buttons(Peer::COMBINED);
    let new_pad = read_pad(Peer::COMBINED).unwrap_or_default();
    let new_dpad = new_pad.as_dpad();

    // If a direction button is held, track for how long.
    if new_dpad.up || new_dpad.down {
        state.held_for += 1;
    } else {
        state.held_for = 0;
    }

    let released_buttons = new_buttons.just_released(&state.old_buttons);
    let command = if released_buttons.s {
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
            play_note(audio::Freq::A4);
        }
        Command::GoUp => {
            state.pos = state.pos.saturating_sub(1);
            play_note(audio::Freq::AS4);
        }
        Command::Launch => {
            let Some(app) = state.apps.get(state.pos) else {
                return;
            };
            let splash_path = alloc::format!("roms/{}/{}/_splash", &app.author_id, &app.id);
            state.splash = Some(splash_path);
            sudo::run_app(&app.author_id, &app.id);
        }
    }
}

/// Show message about no apps (except launcher itself) installed.
fn render_empty(state: &State) {
    let font = state.font.as_font();
    let point = Point::new(62, HEIGHT / 2 - LINE_HEIGHT / 2);
    draw_text("No apps installed", &font, point, Color::DarkBlue);
}

fn play_note(f: audio::Freq) {
    const DURATION: u32 = 140;
    audio::OUT.clear();
    let gain1 = audio::OUT.add_gain(0.5);
    gain1.modulate(audio::LinearModulator {
        start: 0.0,
        end: 0.5,
        start_at: audio::Time::ZERO,
        end_at: audio::Time::ms(DURATION / 2),
    });

    let gain2 = gain1.add_gain(0.5);
    gain2.modulate(audio::LinearModulator {
        start: 0.5,
        end: 0.0,
        start_at: audio::Time::ms(DURATION / 2),
        end_at: audio::Time::ms(DURATION),
    });

    gain2.add_sine(f, 0.);
}
