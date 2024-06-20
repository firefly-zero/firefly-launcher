use crate::*;
use alloc::format;
use firefly_rust::*;

pub fn init(_state: &mut State) {}

pub fn update(_state: &mut State) {}

pub fn render(state: &State) {
    clear_screen(Color::White);
    const MARGIN: i32 = 46;
    let app = &state.apps[state.pos];

    let font = state.font.as_font();
    let corner = Size::new(4, 4);
    let box_style = Style {
        stroke_color: Color::DarkBlue,
        ..Style::default()
    };

    {
        let text = format!(
            "Wanna delete data for\n{}?\n\nThere's no going back.",
            app.name
        );
        let point = Point::new(MARGIN + 6, MARGIN + 16);
        draw_text(&text, &font, point, Color::DarkBlue);
        draw_rounded_rect(
            Point::new(MARGIN, MARGIN),
            Size::new(WIDTH - MARGIN * 2, HEIGHT - MARGIN * 2),
            corner,
            box_style,
        );
    }

    let box_width = 24;
    let y = 96;
    {
        let x = MARGIN + 25;
        let point = Point::new(x + 3, y + 7);
        draw_text("yep", &font, point, Color::DarkBlue);
        draw_rounded_rect(
            Point::new(x, y),
            Size::new(box_width, 12),
            corner,
            box_style,
        );
    }

    {
        let x = MARGIN + 75;
        let point = Point::new(x + 3, y + 7);
        draw_text("nuh", &font, point, Color::DarkBlue);
        draw_rounded_rect(
            Point::new(x, y),
            Size::new(box_width, 12),
            corner,
            box_style,
        );
    }
}
