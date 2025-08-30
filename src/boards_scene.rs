use crate::*;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{boxed::Box, format};
use firefly_rust::*;
use firefly_types::Encode;

const LINE_HEIGHT: i32 = 12;

pub struct BoardInfo {
    name: String,
}

pub fn init(state: &mut State) {
    state.old_buttons = Buttons::default();
    let items = Box::new([("back", Scene::Info), ("exit", Scene::List)]);
    state.button_group = Some(ButtonGroup::new(items));

    let app = &mut state.apps[state.pos];
    app.try_load_stats();
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
    if let Some(button_group) = state.button_group.as_mut() {
        if let Some(scene) = button_group.update() {
            state.transition_to(scene);
        }
    }
}

pub fn render(state: &State) {
    clear_screen(Color::White);
    let app = &state.apps[state.pos];
    let font = state.font.as_font();
    if let Some(boards) = &app.boards {
        for (board, i) in boards.iter().zip(1..) {
            render_board(&font, i, board);
        }
    }
    if let Some(button_group) = &state.button_group {
        button_group.render(&font);
    }
}

fn render_board(font: &Font<'_>, i: i32, b: &BoardInfo) {
    let point = Point::new(6, LINE_HEIGHT * i);
    let color = Color::DarkBlue;
    draw_text(&b.name, font, point, color);
}
