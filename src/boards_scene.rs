use crate::*;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use firefly_rust::*;
use firefly_types::Encode;

const LINE_HEIGHT: i32 = 12;

pub struct BoardInfo {
    pub name: String,
    pos: u16,
    pub min: i16,
    pub max: i16,
    pub time: bool,
    pub decimals: u8,
}

impl Gt for BoardInfo {
    fn gt(&self, other: &Self) -> bool {
        self.pos.gt(&other.pos)
    }
}

pub const fn init(state: &mut State) {
    state.board_pos = 0;
}

pub fn try_load_boards(app: &mut App) {
    if app.boards.is_some() {
        return;
    }
    let boards_path = format!("roms/{}/{}/_boards", app.author_id, app.id);
    let Some(raw) = sudo::load_file_buf(&boards_path) else {
        return;
    };
    let Ok(raw_boards) = firefly_types::Boards::decode(raw.data()) else {
        return;
    };
    let mut boards = Vec::with_capacity(raw_boards.boards.len());
    for info in raw_boards.boards.iter() {
        boards.push(BoardInfo {
            name: info.name.to_owned(),
            pos: info.position,
            min: info.min,
            max: info.max,
            time: info.time,
            decimals: info.decimals,
        });
    }
    bubble_sort(&mut boards);
    app.boards = Some(boards);
}

pub fn update(state: &mut State) {
    let app = &state.apps[state.pos];
    let input = state.input.get();
    if input == Input::Back {
        state.transition_to(Scene::Info);
        return;
    }
    let Some(boards) = &app.boards else {
        return;
    };
    if input == Input::Select {
        state.transition_to(Scene::Scores(state.board_pos as _));
        return;
    }
    if input == Input::Down && state.board_pos < boards.len() - 1 {
        state.board_pos += 1;
    }
    if input == Input::Up && state.board_pos > 0 {
        state.board_pos -= 1;
    }
}

pub fn render(state: &State) {
    clear_screen(Color::White);
    let app = &state.apps[state.pos];
    let font = state.font.as_font();
    let Some(boards) = &app.boards else {
        let point = Point::new(6, LINE_HEIGHT);
        draw_text("the app has no scoreboards", &font, point, Color::Black);
        return;
    };
    let y = LINE_HEIGHT * (state.board_pos as i32 + 1) - 7;
    draw_cursor(y, false, state.input.pressed());
    for (board, i) in boards.iter().zip(1..) {
        render_board(&font, i, board);
    }
}

fn render_board(font: &Font<'_>, i: i32, b: &BoardInfo) {
    let point = Point::new(6, LINE_HEIGHT * i);
    draw_text(&b.name, font, point, Color::Black);
}
