use crate::*;
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;

static FIELDS: &[&str] = &[
    "Author ID:",
    "App ID:",
    "Author name:",
    "App name:",
    "ROM size:",
    "Data size:",
];

pub fn init(state: &mut State) {
    state.old_buttons = Buttons::default()
}

pub fn update(state: &mut State) {
    let new_buttons = read_buttons(Player::P0);
    let released_buttons = new_buttons.just_released(&state.old_buttons);
    if released_buttons.any() {
        state.transition_to(Scene::List);
        // open_menu();
    }
    state.old_buttons = new_buttons;
}

pub fn render(state: &State) {
    clear_screen(Color::White);
    let font = state.font.as_font();
    for (text, i) in FIELDS.iter().zip(1..) {
        let point = Point::new(6, LINE_HEIGHT * i);
        draw_text(text, &font, point, Color::DarkBlue);
    }
    let app = &state.apps[state.pos];
    render_info(&font, 1, &app.author_id);
    render_info(&font, 2, &app.id);
    render_info(&font, 3, &app.author_name);
    render_info(&font, 4, &app.name);
}

fn render_info(font: &Font, i: i32, t: &str) {
    let point = Point::new(100, LINE_HEIGHT * i);
    draw_text(t, &font, point, Color::DarkBlue);
}
