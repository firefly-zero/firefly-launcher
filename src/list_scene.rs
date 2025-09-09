use crate::{scroll::ScrollBar, *};
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;
/// How many apps fit on the same page
pub const PER_SCREEN: usize = 12;

pub fn init(state: &mut State) {
    state.apps = read_apps();
}

pub fn update(state: &mut State) {
    let input = state.input.get();
    // Shift and stutter the selection when the first or the last app
    // is selected to indicate that there are no more items on the list.
    state.shift = if state.input.held_for % 30 >= 25 {
        0
    } else if input == Input::Down && state.pos + 1 == state.apps.len() {
        1
    } else if input == Input::Up && state.pos == 0 {
        -1
    } else {
        0
    };
    // Control the scroll speed when the up.down button is held.
    if state.input.held_for > 30 && state.input.held_for % 4 != 0 {
        return;
    }

    match state.input.get() {
        Input::Down => {
            let apps_count = state.apps.len();
            if state.pos + 1 < apps_count {
                state.pos += 1;
            }
            play_note(audio::Freq::A4);
        }
        Input::Up => {
            state.pos = state.pos.saturating_sub(1);
            play_note(audio::Freq::AS4);
        }
        Input::Select => {
            let Some(app) = state.apps.get(state.pos) else {
                return;
            };
            let splash_path = alloc::format!("roms/{}/{}/_splash", &app.author_id, &app.id);
            state.splash = Some(splash_path);
            sudo::run_app(&app.author_id, &app.id);
        }
        Input::Back => {
            state.transition_to(Scene::Info);
        }
        _ => {}
    }

    // If the selection cursor tries to go out of screen,
    // scroll the list to keep the selection on the screen.
    if state.pos > state.top_pos + PER_SCREEN {
        state.top_pos = state.pos - PER_SCREEN;
    } else if state.pos < state.top_pos {
        state.top_pos = state.pos;
    }
}

pub fn render(state: &State) {
    if let Some(splash_path) = &state.splash {
        let splash = sudo::load_file_buf(splash_path);
        if let Some(splash) = splash {
            draw_image(&splash.as_image(), Point::MIN);
        } else {
            let mut buf = alloc::boxed::Box::new([0u8; 9607]);
            let splash = load_file("_splash", &mut buf[..]);
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
    ScrollBar::from_state(state).render();
    // draw_scroll(state);
    draw_online(state);
}

/// Render the list of installed apps.
fn draw_apps(state: &State) {
    let font = state.font.as_font();
    let apps = state.apps.iter().skip(state.top_pos).take(PER_SCREEN + 1);
    for (i, app) in apps.enumerate() {
        let point = Point::new(6, 9 + i as i32 * LINE_HEIGHT);
        draw_text(&app.name, &font, point, Color::Black);
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
        draw_text(&app.author_name, &font, point, Color::Gray);
    }
}

/// Render a selection box around the currently selected app.
fn draw_selection(state: &State) {
    let pos = state.pos.saturating_sub(state.top_pos);
    let has_scroll = state.apps.len() - 1 > PER_SCREEN;
    let y = 2 + pos as i32 * LINE_HEIGHT + state.shift;
    draw_cursor(y, has_scroll);
}

/// Render a green indicator in a corner if the device is connected to other devices.
fn draw_online(state: &State) {
    if !state.is_online {
        return;
    }
    let p = Point::new(WIDTH - 23, HEIGHT - 12);
    let s = Style::solid(Color::Green);
    draw_circle(p, 8, s);
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
