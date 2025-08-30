use crate::*;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use firefly_rust::*;
use firefly_types::Encode;

const LINE_HEIGHT: i32 = 12;

pub struct BoardInfo {
    name: String,
}

pub fn init(state: &mut State) {
    state.old_buttons = Buttons::default();
    state.board_pos = 0;
    let app = &mut state.apps[state.pos];
    try_load_boards(app);
}

fn try_load_boards(app: &mut App) {
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
        });
    }
    // TODO: sort boards
    app.boards = Some(boards);
}

pub fn update(state: &mut State) {
    let app = &state.apps[state.pos];
    let Some(boards) = &app.boards else {
        return;
    };
    let new_dpad = read_pad(Peer::COMBINED).unwrap_or_default().as_dpad();
    let dpad_pressed = new_dpad.just_pressed(&state.old_dpad);
    if dpad_pressed.down && state.board_pos < boards.len() - 1 {
        state.board_pos += 1;
    }
    if dpad_pressed.up && state.board_pos > 0 {
        state.board_pos -= 1;
    }
    state.old_dpad = new_dpad;
}

pub fn render(state: &State) {
    clear_screen(Color::White);
    let app = &state.apps[state.pos];
    let font = state.font.as_font();
    let Some(boards) = &app.boards else {
        return;
    };
    {
        let y = LINE_HEIGHT * (state.board_pos as i32 + 1) - 7;
        draw_cursor(y, false);
    }
    for (board, i) in boards.iter().zip(1..) {
        render_board(&font, i, board);
    }
}

fn render_board(font: &Font<'_>, i: i32, b: &BoardInfo) {
    let point = Point::new(6, LINE_HEIGHT * i);
    draw_text(&b.name, font, point, Color::Black);
}
