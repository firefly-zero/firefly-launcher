use firefly_rust::*;

use crate::{list_scene::PER_SCREEN, state::State};

/// The distance on top and bottom between scrollbar and screen boundaries.
const MARGIN_VERT: i32 = 3;
const MARGIN_RIGHT: i32 = 4;
const BAR_WIDTH: i32 = 6;
const THUMB_HEIGHT: i32 = 10;
/// The x coordinate of the bar's middle.
const MIDDLE_X: i32 = WIDTH - BAR_WIDTH / 2 - MARGIN_RIGHT;
/// Half of the bar width.
const HALF_WIDTH: i32 = BAR_WIDTH / 2;
const ARROW_HEIGHT: i32 = HALF_WIDTH;

pub struct ScrollBar {
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
            top_pos: state.top_pos,
            per_page: PER_SCREEN,
            total: state.apps.len(),
        }
    }

    pub fn render(&self) {
        self.draw_arrow_up();
        self.draw_arrow_down();
    }

    fn draw_arrow_up(&self) {
        if self.top_pos == 0 {
            return;
        }
        let left = Point::new(MIDDLE_X - HALF_WIDTH, MARGIN_VERT + ARROW_HEIGHT);
        let right = Point::new(MIDDLE_X + HALF_WIDTH, MARGIN_VERT + ARROW_HEIGHT);
        let top = Point::new(MIDDLE_X, MARGIN_VERT);
        draw_triangle(left, top, right, style());
    }

    fn draw_arrow_down(&self) {
        if self.total - 1 <= self.top_pos + self.per_page {
            return;
        }
        let left = Point::new(MIDDLE_X - HALF_WIDTH, HEIGHT - MARGIN_VERT - ARROW_HEIGHT);
        let right = Point::new(MIDDLE_X + HALF_WIDTH, HEIGHT - MARGIN_VERT - ARROW_HEIGHT);
        let bottom = Point::new(MIDDLE_X, HEIGHT - MARGIN_VERT);
        draw_triangle(left, bottom, right, style());
    }
}

fn style() -> Style {
    Style {
        fill_color: Color::DarkBlue,
        ..Style::default()
    }
}
