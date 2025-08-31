use crate::*;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;

pub struct ScoreInfo {
    name: String,
    value: String,
}

pub fn init(state: &mut State, i: u8) {
    state.board_pos = 0;
    let app = &mut state.apps[state.pos];
    app.try_load_stats();
    load_scores(app, i);
}

pub fn load_scores(app: &mut App, i: u8) {
    app.scores = None;
    let Some(stats) = &app.stats else {
        return;
    };
    let Some(raw_scores) = stats.scores.get(usize::from(i)) else {
        return;
    };
    let Some(boards) = &app.boards else {
        return;
    };
    let Some(board) = boards.get(usize::from(i)) else {
        return;
    };

    let mut scores = Vec::new();
    let my_name = "me";
    for score in raw_scores.me.iter() {
        let score = *score;
        if score < board.min {
            continue;
        }
        if score < board.max {
            continue;
        }
        scores.push(ScoreInfo {
            name: my_name.to_owned(),
            value: format_value(score, board),
        });
    }
    app.scores = Some(scores);
}

pub fn update(_state: &mut State, _i: u8) {
    // ...
}

pub fn render(state: &State, _: u8) {
    clear_screen(Color::White);
    let font = state.font.as_font();
    let app = &state.apps[state.pos];
    let Some(scores) = &app.scores else {
        return;
    };
    let mut i = 0;
    for score in scores {
        i += 1;
        let point = Point::new(6, LINE_HEIGHT * i);
        draw_text(&score.name, &font, point, Color::Black);
        let point = Point::new(100, LINE_HEIGHT * i);
        draw_text(&score.value, &font, point, Color::Black);
    }
}

fn format_value(value: i16, _board: &BoardInfo) -> String {
    format!("{value}")
}
