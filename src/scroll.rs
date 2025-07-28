use firefly_rust::*;

use crate::{list_scene::PER_SCREEN, state::State};

/// The distance on top and bottom between scrollbar and screen boundaries.
const MARGIN_VERT: i32 = 3;
const MARGIN_RIGHT: i32 = 4;
const BAR_WIDTH: i32 = 6;
const THUMB_HEIGHT: i32 = 10;

/// The x coordinate of the bar's middle.
const MIDDLE_X: i32 = WIDTH - BAR_WIDTH / 2 - MARGIN_RIGHT;
const LEFT_X: i32 = MIDDLE_X - HALF_WIDTH;
const RIGHT_X: i32 = MIDDLE_X + HALF_WIDTH;
/// Half of the bar width.
const HALF_WIDTH: i32 = BAR_WIDTH / 2;
const ARROW_HEIGHT: i32 = HALF_WIDTH;
const TRACK_MARGIN_VERT: i32 = MARGIN_VERT * 2 + ARROW_HEIGHT;
const TRACK_HEIGHT: i32 = HEIGHT - TRACK_MARGIN_VERT * 2;

pub struct ScrollBar {
    pos: usize,
    /// Index of the top item on the current page.
    top_pos: usize,
    /// How many items fit per page.
    per_page: usize,
    /// How many items the list contains in total accross all pages.
    total: usize,
}

impl ScrollBar {
    pub fn from_state(state: &State) -> Self {
        Self {
            pos: state.pos,
            top_pos: state.top_pos,
            per_page: PER_SCREEN,
            total: state.apps.len(),
        }
    }

    pub fn render(&self) {
        self.draw_arrow_up();
        self.draw_arrow_down();
        self.draw_track();
        self.draw_thumb();
    }

    fn draw_arrow_up(&self) {
        if self.top_pos == 0 {
            return;
        }
        let left = Point::new(LEFT_X, MARGIN_VERT + ARROW_HEIGHT);
        let right = Point::new(RIGHT_X, MARGIN_VERT + ARROW_HEIGHT);
        let top = Point::new(MIDDLE_X, MARGIN_VERT);
        draw_triangle(left, top, right, style());
    }

    fn draw_arrow_down(&self) {
        if self.total - 1 <= self.top_pos + self.per_page {
            return;
        }
        let left = Point::new(LEFT_X, HEIGHT - MARGIN_VERT - ARROW_HEIGHT);
        let right = Point::new(RIGHT_X, HEIGHT - MARGIN_VERT - ARROW_HEIGHT);
        let bottom = Point::new(MIDDLE_X, HEIGHT - MARGIN_VERT);
        draw_triangle(left, bottom, right, style());
    }

    fn draw_track(&self) {
        if self.total <= self.per_page {
            return;
        }
        let style = LineStyle {
            color: Color::LightGray,
            width: 1,
        };
        let a = Point::new(LEFT_X, TRACK_MARGIN_VERT);
        let b = Point::new(RIGHT_X, TRACK_MARGIN_VERT);
        draw_line(a, b, style);

        let a = Point::new(LEFT_X, TRACK_HEIGHT + TRACK_MARGIN_VERT);
        let b = Point::new(RIGHT_X, TRACK_HEIGHT + TRACK_MARGIN_VERT);
        draw_line(a, b, style);
    }

    fn draw_thumb(&self) {
        if self.total <= self.per_page {
            return;
        }
        let run_pix = (TRACK_HEIGHT - THUMB_HEIGHT) as usize;
        let mut y = TRACK_MARGIN_VERT + (run_pix * self.pos / (self.total - 1)) as i32;
        if self.pos == 0 {
            y += 1;
        }
        draw_rect(
            Point::new(LEFT_X, y),
            Size::new(BAR_WIDTH + 1, THUMB_HEIGHT),
            Style {
                fill_color: Color::DarkBlue,
                ..Style::default()
            },
        );
    }
}

fn style() -> Style {
    Style {
        fill_color: Color::LightGray,
        ..Style::default()
    }
}
