use crate::*;
use firefly_ui::*;

pub fn update(state: &mut State) {
    if matches!(state.input.get(), Input::Back | Input::Select) {
        state.transition_to(Scene::List);
    }
}

pub fn render(state: &State, msg: &str) {
    let theme = state.settings.theme;
    draw_bg_grid(theme);
    draw_dialog(theme, &state.font, msg, &["ok"], 0, state.input.pressed());
}
