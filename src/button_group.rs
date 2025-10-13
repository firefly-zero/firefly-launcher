use crate::*;
use alloc::boxed::Box;
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;

pub struct ButtonGroup {
    pub items: Box<[(&'static str, Scene)]>,
    pub cursor: usize,
}

impl ButtonGroup {
    pub const fn new(items: Box<[(&'static str, Scene)]>) -> Self {
        Self { cursor: 0, items }
    }

    pub fn update(&mut self, input: &InputManager) -> Option<Scene> {
        let input = input.get();
        if input == Input::Back {
            return Some(Scene::List);
        } else if input == Input::Select {
            let selected = self.items[self.cursor];
            return Some(selected.1);
        } else if input == Input::Down && self.cursor < self.items.len() - 1 {
            self.cursor += 1;
        } else if input == Input::Up && self.cursor > 0 {
            self.cursor -= 1;
        }
        None
    }

    pub fn render(&self, font: &Font<'_>) {
        let buttons = read_buttons(Peer::COMBINED);
        let pressed = buttons.s || buttons.e;
        let n = self.items.len() as i32;
        {
            let i = n - self.cursor as i32;
            let y = HEIGHT - LINE_HEIGHT * i + LINE_HEIGHT / 2 - 8;
            draw_cursor(y, false, pressed);
        }
        for ((item, _), i) in self.items.iter().zip(0..) {
            render_button(font, n - i, item);
        }
    }
}

fn render_button(font: &Font<'_>, i: i32, t: &str) {
    let point = Point::new(6, HEIGHT - LINE_HEIGHT * i + LINE_HEIGHT / 2 - 1);
    draw_text(t, font, point, Color::Black);
}
