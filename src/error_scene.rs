use crate::*;
use firefly_sudo::sudo;
use firefly_ui::*;

pub fn update(state: &mut State, disconnect: bool) {
    if matches!(state.input.get(), Input::Back | Input::Select) {
        if disconnect {
            sudo::run_app("sys", "disconnector");
        } else {
            state.transition_to(Scene::List);
        }
    }
}

pub fn render(state: &State, msg: &str) {
    let theme = state.settings.theme;
    draw_bg_grid(theme);
    draw_dialog(theme, &state.font, msg, &["ok"], 0, state.input.pressed());
}
