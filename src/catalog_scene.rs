use crate::*;

const BASE_URL: &str = "https://catalog.fireflyzero.com/";
const QR_WIDTH: i32 = 33;

pub const fn init(_state: &State) {}

pub fn update(state: &mut State) {
    let input = state.input.get();
    if input != Input::None {
        state.transition_to(Scene::Info);
    }
}

pub fn render(state: &State) {
    clear_screen(Color::White);
    let app = &state.apps[state.pos];
    let url = alloc::format!("{BASE_URL}{}.{}", app.author_id, app.id);
    let point = Point::new((WIDTH - QR_WIDTH) / 2, HEIGHT / 2 - QR_WIDTH);
    draw_qr(&url, point, Color::Black, Color::White);
    let font = state.font.as_font();

    let y = point.y + QR_WIDTH + i32::from(font.char_height()) + 2;
    draw_centered_text(&font, "catalog.fireflyzero.com/", y);

    let y = y + i32::from(font.char_height());
    let id = alloc::format!("{}.{}", app.author_id, app.id);
    draw_centered_text(&font, &id, y);
}

fn draw_centered_text(font: &Font<'_>, text: &str, y: i32) {
    let x = (WIDTH - font.line_width(text) as i32) / 2;
    let point = Point::new(x, y);
    draw_text(text, font, point, Color::Black);
}
