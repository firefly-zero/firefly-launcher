use crate::*;
use alloc::{boxed::Box, format};
use firefly_rust::*;
use firefly_types::Encode;

const LINE_HEIGHT: i32 = 12;

static COLUMNS: &[&str] = &["1p", "2p", "3p", "4p"];
static FIELDS: &[&str] = &["launches", "installed", "updated"];

pub fn init(state: &mut State) {
    state.old_buttons = Buttons::default();
    let items = Box::new([("back", Scene::Info), ("exit", Scene::List)]);
    state.button_group = Some(ButtonGroup::new(items));

    let app = &mut state.apps[state.pos];
    if app.stats.is_none() {
        let stats_path = format!("data/{}/{}/stats", app.author_id, app.id);
        // TODO: don't unwrap
        let raw = sudo::load_file_buf(&stats_path).unwrap();
        let stats = firefly_types::Stats::decode(raw.data()).unwrap();
        app.stats = Some(stats);
    }
}

pub fn update(state: &mut State) {
    if let Some(button_group) = state.button_group.as_mut() {
        if let Some(scene) = button_group.update() {
            state.transition_to(scene);
        }
    }
}

pub fn render(state: &State) {
    clear_screen(Color::White);
    let font = state.font.as_font();
    for (text, i) in COLUMNS.iter().zip(0..) {
        let point = Point::new(100 + 30 * i, LINE_HEIGHT);
        draw_text(text, &font, point, Color::DarkBlue);
    }
    for (text, i) in FIELDS.iter().zip(2..) {
        let point = Point::new(6, LINE_HEIGHT * i);
        draw_text(text, &font, point, Color::DarkBlue);
    }
    let app = &state.apps[state.pos];
    if let Some(stats) = &app.stats {
        for (n, i) in stats.launches.iter().zip(0..) {
            render_info(&font, 2, i, &format!("{n}"));
        }
        let installed_on = format_date(stats.installed_on);
        render_info(&font, 3, 0, &installed_on);
        let updated_on = format_date(stats.updated_on);
        render_info(&font, 4, 0, &updated_on);
    }
    if let Some(button_group) = &state.button_group {
        button_group.render(&font);
    }
}

fn render_info(font: &Font<'_>, i: i32, j: i32, t: &str) {
    let point = Point::new(100 + j * 30, LINE_HEIGHT * i);
    draw_text(t, font, point, Color::Blue);
}
