use crate::scroll::ScrollBar;
use crate::*;
use alloc::vec;
use firefly_rust::*;
use firefly_sudo::sudo;

const LINE_HEIGHT: i32 = 12;
/// How many apps fit on the same page
pub const PER_SCREEN: usize = 12;

pub const fn init(_state: &mut State) {}

pub fn update(state: &mut State) {
    if state.input.get() == Input::None {
        state.idle += 1;
    } else {
        state.idle = 0;
    }
    let hitting_wall = state.pos == 0 || state.pos + 1 == state.apps.len();
    state.shift = i32::from(state.input.jitter(hitting_wall));
    match state.input.get() {
        Input::Down => {
            if state.pos + 1 != state.apps.len() {
                play_note(audio::Freq::A4);
            }
            let apps_count = state.apps.len();
            if state.pos + 1 < apps_count {
                state.pos += 1;
            }
        }
        Input::Up => {
            if state.pos != 0 {
                play_note(audio::Freq::AS4);
            }
            state.pos = state.pos.saturating_sub(1);
        }
        Input::Left => {
            if state.pos != 0 {
                play_note(audio::Freq::AS4);
            }
            if state.pos == state.top_pos {
                state.pos = state.pos.saturating_sub(PER_SCREEN);
            } else {
                state.pos = state.top_pos;
            }
        }
        Input::Right => {
            if state.pos + 1 != state.apps.len() {
                play_note(audio::Freq::A4);
            }
            if state.pos == state.top_pos + PER_SCREEN {
                state.pos = usize::min(state.pos + PER_SCREEN, state.apps.len() - 1);
            } else {
                state.pos = (state.top_pos + PER_SCREEN).min(state.apps.len() - 1);
            }
        }
        Input::Select => {
            launch_selected(state);
        }
        Input::Back => {
            state.transition_to(Scene::Info);
        }
        Input::None => {}
    }

    // If the selection cursor tries to go out of screen,
    // scroll the list to keep the selection on the screen.
    if state.pos > state.top_pos + PER_SCREEN {
        state.top_pos = state.pos - PER_SCREEN;
    } else if state.pos < state.top_pos {
        state.top_pos = state.pos;
    }
}

pub fn launch_selected(state: &mut State) {
    let Some(app) = state.apps.get(state.pos) else {
        return;
    };
    let updates = app.author_id == "sys" && app.id == "updates";
    if updates {
        state.transition_to(Scene::Delegate("sys", "manuals"));
        return;
    }
    let disconnector = app.author_id == "sys" && app.id == "disconnector";
    if !disconnector {
        let splash_path = alloc::format!("roms/{}/{}/_splash", &app.author_id, &app.id);
        state.splash = Some(splash_path);
    }
    sudo::run_app(&app.author_id, &app.id);
}

pub fn render(state: &State) {
    if let Some(splash_path) = &state.splash {
        draw_splash(splash_path);
        return;
    }
    clear_screen(state.settings.theme.bg);
    if state.apps.is_empty() {
        render_empty(state);
        return;
    }
    draw_selection(state);
    draw_apps(state);
    ScrollBar::from_state(state).render();
    draw_online(state);
    draw_mouse(state);
}

/// If you leave the list view idle for 2m, a tiny mouse will run across the screen.
fn draw_mouse(state: &State) {
    const SW: i32 = 16;
    const SH: i32 = 9;
    const DELAY: i32 = 2 * 3600; // 2 minutes

    if !state.settings.easter_eggs {
        return;
    }
    let frame = state.idle - DELAY;
    if !(0..=300).contains(&frame) {
        return;
    }
    let sub_p = Point::new((frame / 8 % 4) * SW, 0);
    let sub = state.mouse.sub(sub_p, Size::new(SW, SH));
    draw_sub_image(&sub, Point::new(frame - SW, HEIGHT - SH));
}

/// When an app is about to be launched, render its splash screen.
fn draw_splash(splash_path: &str) {
    let splash = sudo::load_file_buf(splash_path);
    if let Some(splash) = splash {
        draw_image(&splash.into_image(), Point::MIN);
    } else {
        let mut buf = vec![0u8; 19204].into_boxed_slice();
        let splash = load_file("_splash", &mut buf[..]);
        draw_image(&splash.into_image(), Point::MIN);
    }
}

/// Render the list of installed apps.
fn draw_apps(state: &State) {
    let apps = state.apps.iter().skip(state.top_pos).take(PER_SCREEN + 1);
    let theme = state.settings.theme;
    for (i, app) in apps.enumerate() {
        let mut point = Point::new(6, 9 + i as i32 * LINE_HEIGHT);
        let selected = state.top_pos + i == state.pos;
        if selected && state.input.pressed() {
            point.x += 1;
            point.y += 1;
        }
        draw_text(&app.name, &state.font, point, theme.primary);

        // Notification icon.
        let sub_p = if let Some(notif) = &app.notif {
            if notif.badges && notif.boards {
                Some(Point::new(0, 0))
            } else if notif.badges {
                Some(Point::new(32, 0))
            } else if notif.boards {
                Some(Point::new(64, 0))
            } else if notif.manual {
                Some(Point::new(48, 0))
            } else {
                None
            }
        } else {
            Some(Point::new(16, 0))
        };
        if let Some(sub_p) = sub_p {
            let size = Size::new(16, 14);
            let sub = state.icons.sub(sub_p, size);
            draw_sub_image(&sub, Point::new(200, point.y - 8));
            continue;
        }

        // Don't show the author name
        // if the app name takes more than half of the screen.
        if app.name.len() > 19 {
            continue;
        }
        // Don't show the author name if it doesn't fit on the screen.
        if app.author_name.len() > 16 {
            continue;
        }
        point.x += WIDTH / 2;
        let color = if state.settings.contrast {
            theme.accent
        } else {
            theme.secondary
        };
        draw_text(&app.author_name, &state.font, point, color);
    }
}

/// Render a selection box around the currently selected app.
fn draw_selection(state: &State) {
    let pos = state.pos.saturating_sub(state.top_pos);
    let has_scroll = state.apps.len() - 1 > PER_SCREEN;
    let y = 2 + pos as i32 * LINE_HEIGHT + state.shift;
    draw_cursor(y, has_scroll, state.input.pressed(), &state.settings.theme);
}

/// Render a green indicator in a corner if the device is connected to other devices.
fn draw_online(state: &State) {
    if !state.is_online {
        return;
    }
    let p = Point::new(WIDTH - 23, HEIGHT - 12);
    let s = Style::solid(state.settings.theme.accent);
    draw_circle(p, 8, s);
}

/// Show message about no apps (except launcher itself) installed.
///
/// Shouldn't be reachable on a normal installation (but can be hit on dev
/// environments) because we pre-install sys apps and don't let them be removed.
fn render_empty(state: &State) {
    let point = Point::new(62, HEIGHT / 2 - LINE_HEIGHT / 2);
    let color = state.settings.theme.primary;
    draw_text("No apps installed", &state.font, point, color);
}

fn play_note(f: audio::Freq) {
    const DURATION_MS: u32 = 30;
    audio::OUT.clear();
    let gain_up = audio::OUT.add_gain(0.5);
    let modulator = audio::LinearModulator {
        start_at: audio::Time::ZERO,
        end_at: audio::ms(DURATION_MS / 2),
    };
    gain_up.modulate(0.0, 0.5, modulator);

    let gain_down = gain_up.add_gain(0.5);
    let modulator = audio::LinearModulator {
        start_at: audio::ms(DURATION_MS / 2),
        end_at: audio::ms(DURATION_MS),
    };
    gain_down.modulate(0.5, 0.0, modulator);

    gain_down.add_sine(f, 0.);
}
