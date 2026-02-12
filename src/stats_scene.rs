use crate::*;
use alloc::boxed::Box;
use alloc::format;
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;

static COLUMNS: &[&str] = &["1p", "2p", "3p", "4p"];
static FIELDS: &[Message] = &[Message::Launches, Message::Installed, Message::Updated];

pub fn init(state: &mut State) {
    let lang = state.settings.language;
    let items = Box::new([
        (Message::Back.translate(lang), Scene::Info),
        (Message::Exit.translate(lang), Scene::List),
    ]);
    state.button_group = Some(ButtonGroup::new(items));

    let app = &mut state.apps[state.pos];
    app.try_load_stats();
}

pub fn update(state: &mut State) {
    if let Some(button_group) = state.button_group.as_mut() {
        if let Some(scene) = button_group.update(&state.input) {
            state.transition_to(scene);
        }
    }
}

pub fn render(state: &State) {
    let theme = state.settings.theme;
    clear_screen(theme.bg);
    let font = state.font.as_font();
    for (text, i) in COLUMNS.iter().zip(0..) {
        let point = Point::new(100 + 30 * i, LINE_HEIGHT);
        draw_text(text, &font, point, theme.primary);
    }
    for (msg, i) in FIELDS.iter().zip(2..) {
        let point = Point::new(6, LINE_HEIGHT * i);
        let text = msg.translate(state.settings.language);
        draw_text(text, &font, point, theme.primary);
    }
    let app = &state.apps[state.pos];
    if let Some(stats) = &app.stats {
        for (n, i) in stats.launches.iter().zip(0..) {
            render_info(&font, &theme, 2, i, &format!("{n}"));
        }
        let installed_on = format_date(stats.installed_on);
        render_info(&font, &theme, 3, 0, &installed_on);
        let updated_on = format_date(stats.updated_on);
        render_info(&font, &theme, 4, 0, &updated_on);
    }
    if let Some(button_group) = &state.button_group {
        button_group.render(&font, &state.settings.theme);
    }
}

fn render_info(font: &Font<'_>, theme: &Theme, i: i32, j: i32, t: &str) {
    let point = Point::new(100 + j * 30, LINE_HEIGHT * i);
    draw_text(t, font, point, theme.accent);
}
