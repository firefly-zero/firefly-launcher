use crate::*;
use alloc::boxed::Box;

const BASE_URL: &str = "https://catalog.fireflyzero.com/";
const QR_WIDTH: i32 = 33;

pub fn init(state: &mut State) {
    let items = Box::new([("back", Scene::Info), ("exit", Scene::List)]);
    state.button_group = Some(ButtonGroup::new(items));
}

pub fn update(state: &mut State) {
    if let Some(button_group) = state.button_group.as_mut() {
        if let Some(scene) = button_group.update(&state.input) {
            state.transition_to(scene);
        }
    }
}

pub fn render(state: &State) {
    clear_screen(state.settings.theme.bg);
    let font = state.font.as_font();
    let theme = state.settings.theme;

    if let Some(button_group) = &state.button_group {
        button_group.render(&font, &theme);
    }

    let app = &state.apps[state.pos];
    let url = alloc::format!("{BASE_URL}{}.{}", app.author_id, app.id);
    let point = Point::new((WIDTH - QR_WIDTH) / 2, HEIGHT / 2 - QR_WIDTH - 4);
    draw_qr(&url, point, Color::Black, Color::White);

    let y = point.y + QR_WIDTH + i32::from(font.char_height()) + 8;
    draw_centered_text(&font, &theme, "catalog.fireflyzero.com/", y);

    let y = y + i32::from(font.char_height());
    let id = alloc::format!("{}.{}", app.author_id, app.id);
    draw_centered_text(&font, &theme, &id, y);
}

fn draw_centered_text(font: &Font<'_>, theme: &Theme, text: &str, y: i32) {
    let x = (WIDTH - font.line_width(text) as i32) / 2;
    let point = Point::new(x, y);
    draw_text(text, font, point, theme.primary);
}
