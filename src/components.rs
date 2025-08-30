use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;
const MARGIN: i32 = 3;

pub fn draw_cursor(y: i32, has_scroll: bool) {
    let point = Point::new(MARGIN, y);
    let mut size = Size::new(WIDTH - MARGIN * 2, LINE_HEIGHT);
    if has_scroll {
        size.width -= 10;
    }
    let corner = Size::new(4, 4);

    // draw shadow
    {
        let point = point + Point::new(1, 1);
        let style = Style {
            fill_color: Color::Black,
            stroke_color: Color::None,
            stroke_width: 0,
        };
        draw_rounded_rect(point, size, corner, style);
    }

    let style = Style {
        fill_color: Color::LightGreen,
        stroke_color: Color::Black,
        stroke_width: 1,
    };
    draw_rounded_rect(point, size, corner, style);
}
