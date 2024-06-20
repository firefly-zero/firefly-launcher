use crate::*;
use firefly_rust::*;

pub fn init(state: &mut State) {
    state.old_buttons = Buttons::default()
}

pub fn update(state: &mut State) {
    let new_buttons = read_buttons(Player::P0);
    let released_buttons = new_buttons.just_released(&state.old_buttons);
    if released_buttons.any() {
        state.transition_to(Scene::List);
        open_menu();
    }
    state.old_buttons = new_buttons;
}

pub fn render(state: &State) {
    clear_screen(Color::White);
    let font = state.font.as_font();
    let point = Point::new(62, HEIGHT / 2 - 12 / 2);
    draw_text("TODO", &font, point, Color::DarkBlue);
}
