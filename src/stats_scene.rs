use crate::*;
use alloc::format;
use firefly_rust::*;
use firefly_types::Encode;

const LINE_HEIGHT: i32 = 12;

static FIELDS: &[&str] = &["Launches:"];

pub fn init(state: &mut State) {
    state.old_buttons = Buttons::default();
}
pub fn update(state: &mut State) {
    let new_buttons = read_buttons(Peer::COMBINED);
    let released_buttons = new_buttons.just_released(&state.old_buttons);
    if released_buttons.e {
        state.transition_to(Scene::List);
    } else if released_buttons.s {
        state.transition_to(Scene::Info);
    }
    state.old_buttons = new_buttons;

    let app = &mut state.apps[state.pos];
    if app.stats.is_none() {
        let stats_path = format!("data/{}/{}/stats", app.author_id, app.id);
        // TODO: don't unwrap
        let raw = sudo::load_file_buf(&stats_path).unwrap();
        let stats = firefly_types::Stats::decode(raw.data()).unwrap();
        app.stats = Some(stats);
    }
}

pub fn render(state: &State) {
    clear_screen(Color::White);
    let font = state.font.as_font();
    for (text, i) in FIELDS.iter().zip(1..) {
        let point = Point::new(6, LINE_HEIGHT * i);
        draw_text(text, &font, point, Color::DarkBlue);
    }
    let app = &state.apps[state.pos];
    if let Some(stats) = &app.stats {
        render_info(&font, 1, &format!("{}", stats.launches[0]));
    }
    render_button(&font, 0, "back");
}

fn render_info(font: &Font<'_>, i: i32, t: &str) {
    let point = Point::new(100, LINE_HEIGHT * i);
    draw_text(t, font, point, Color::Blue);
}

fn render_button(font: &Font<'_>, i: i32, t: &str) {
    let point = Point::new(6, 4 + LINE_HEIGHT * (i + 8));
    draw_text(t, font, point, Color::DarkBlue);

    let size = Size::new(WIDTH - 8, LINE_HEIGHT);
    let corner = Size::new(4, 4);
    let style = Style {
        fill_color: Color::None,
        stroke_color: Color::DarkBlue,
        stroke_width: 1,
    };
    let point = point - Point::new(2, 8);
    draw_rounded_rect(point, size, corner, style);
}
