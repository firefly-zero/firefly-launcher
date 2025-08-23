use crate::*;
use alloc::boxed::Box;
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;

pub struct ButtonGroup {
    pub cursor: usize,
    pub old_buttons: Buttons,
    pub items: Box<[(&'static str, Scene)]>,
}

impl ButtonGroup {
    pub fn new(items: Box<[(&'static str, Scene)]>) -> Self {
        Self {
            cursor: 0,
            old_buttons: Buttons::default(),
            items,
        }
    }

    pub fn update(&mut self) -> Option<Scene> {
        let new_buttons = read_buttons(Peer::COMBINED);
        let released_buttons = new_buttons.just_released(&self.old_buttons);
        if released_buttons.e {
            return Some(Scene::List);
        } else if released_buttons.s {
            // state.transition_to(Scene::ClearData);
            return Some(Scene::Stats);
        }
        self.old_buttons = new_buttons;
        None
    }

    pub fn render(&self, font: &Font<'_>) {
        for (i, (item, _)) in self.items.iter().enumerate() {
            render_button(font, i as i32, item);
            if i == self.cursor {
                render_cursor(i as i32);
            }
        }
    }
}

fn render_button(font: &Font<'_>, i: i32, t: &str) {
    let point = Point::new(6, 4 + LINE_HEIGHT * (i + 7));
    draw_text(t, font, point, Color::DarkBlue);
}

fn render_cursor(i: i32) {
    let point = Point::new(6, 4 + LINE_HEIGHT * (i + 7));
    let size = Size::new(WIDTH - 8, LINE_HEIGHT);
    let corner = Size::new(4, 4);
    let style = Style {
        fill_color: Color::None,
        stroke_color: Color::DarkBlue,
        stroke_width: 1,
    };
    let point = point - Point::new(2, 8);
    draw_rounded_rect(point, size, corner, style);
}
