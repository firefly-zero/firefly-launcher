use crate::*;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::{String, ToString};
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
    let i = usize::from(i);
    app.scores = None;
    let Some(stats) = &app.stats else {
        return;
    };
    let Some(raw_scores) = stats.scores.get(i) else {
        return;
    };
    let Some(boards) = &app.boards else {
        return;
    };
    let Some(board) = boards.get(i) else {
        return;
    };

    let mut scores = Vec::new();
    let my_name = "me";
    for score in raw_scores.me.iter() {
        let score = *score;
        if score == 0 {
            continue;
        }
        if score < board.min {
            continue;
        }
        if score > board.max {
            continue;
        }
        let val = score.unsigned_abs();
        let value: String = if board.time {
            format_time(val)
        } else if board.decimals > 0 {
            format_decimal(val, board.decimals)
        } else {
            val.to_string()
        };
        scores.push(ScoreInfo {
            name: my_name.to_owned(),
            value,
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
        let point = Point::new(140, LINE_HEIGHT * i);
        draw_text(&score.value, &font, point, Color::Black);
    }
}

fn format_decimal(v: u16, prec: u8) -> String {
    let sep = (10u32).pow(u32::from(prec));
    let right = u32::from(v) % sep;
    let left = u32::from(v) / sep;
    format!("{left}.{right:00$}", usize::from(prec))
}

fn format_time(mut v: u16) -> String {
    let mut parts = Vec::new();
    while v > 0 {
        parts.push(format!("{:02}", v % 60));
        v /= 60;
    }
    reverse(&mut parts);
    parts.join(":")
}

fn reverse(parts: &mut [String]) {
    let size = parts.len();
    for i in 0..(size / 2) {
        parts.swap(i, size - i - 1);
    }
}
