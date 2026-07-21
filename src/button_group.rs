use crate::*;
use alloc::boxed::Box;
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;

pub struct Button {
    pub text: &'static str,
    pub scene: Scene,
    pub accent: bool,
}

pub struct ButtonGroup {
    pub items: Box<[Button]>,
    pub cursor: usize,
}

impl ButtonGroup {
    pub const fn new(items: Box<[Button]>) -> Self {
        Self { cursor: 0, items }
    }

    pub fn update(&mut self, input: &InputManager) -> Option<Scene> {
        match input.get() {
            Input::Back => {
                return Some(Scene::List);
            }
            Input::Select => {
                let selected = &self.items[self.cursor];
                return Some(selected.scene);
            }
            Input::Down if self.cursor < self.items.len() - 1 => {
                self.cursor += 1;
            }
            Input::Up if self.cursor > 0 => {
                self.cursor -= 1;
            }
            Input::Left => {
                self.cursor = 0;
            }
            Input::Right => {
                self.cursor = self.items.len() - 1;
            }
            _ => {}
        }
        None
    }

    pub fn render(&self, font: &FontBuf, theme: &Theme) {
        let buttons = read_buttons(Peer::COMBINED);
        let pressed = buttons.s || buttons.e;
        let n = self.items.len() as i32;
        {
            let i = n - self.cursor as i32;
            let y = HEIGHT - LINE_HEIGHT * i + LINE_HEIGHT / 2 - 8;
            draw_cursor(y, 49, pressed, theme);
        }
        for (button, i) in self.items.iter().zip(0..) {
            let selected = i == self.cursor as i32;
            let i = n - i;
            let y = HEIGHT - LINE_HEIGHT * i + LINE_HEIGHT / 2 - 1;
            let mut point = Point::new(6, y);
            if selected && pressed {
                point.x += 1;
                point.y += 1;
            }
            let color = if button.accent {
                theme.accent
            } else {
                theme.primary
            };
            draw_text(button.text, font, point, color);
        }
    }
}
