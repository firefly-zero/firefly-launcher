use crate::*;
use alloc::boxed::Box;
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;

pub struct ButtonGroup {
    pub items: Box<[(&'static str, Scene)]>,
    pub cursor: usize,
    pub old_buttons: Buttons,
    pub old_dpad: DPad,
}

impl ButtonGroup {
    pub fn new(items: Box<[(&'static str, Scene)]>) -> Self {
        Self {
            cursor: 0,
            old_buttons: Buttons::default(),
            old_dpad: DPad::default(),
            items,
        }
    }

    pub fn update(&mut self) -> Option<Scene> {
        let new_buttons = read_buttons(Peer::COMBINED);
        let released_buttons = new_buttons.just_released(&self.old_buttons);
        if released_buttons.e {
            return Some(Scene::List);
        } else if released_buttons.s {
            let selected = self.items[self.cursor];
            return Some(selected.1);
        }
        self.old_buttons = new_buttons;

        let new_dpad = read_pad(Peer::COMBINED).unwrap_or_default().as_dpad();
        let dpad_pressed = new_dpad.just_pressed(&self.old_dpad);
        if dpad_pressed.down && self.cursor < self.items.len() - 1 {
            self.cursor += 1;
        }
        if dpad_pressed.up && self.cursor > 0 {
            self.cursor -= 1;
        }
        self.old_dpad = new_dpad;

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
