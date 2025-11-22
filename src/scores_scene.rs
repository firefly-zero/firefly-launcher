use crate::*;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;

pub struct ScoreInfo {
    name: String,
    value: String,
    raw_value: i16,
    me: bool,
}

impl Gt for ScoreInfo {
    fn gt(&self, other: &Self) -> bool {
        // We use lt instead of gt to make sure high scores go first.
        self.raw_value.lt(&other.raw_value)
    }
}

pub fn init(state: &mut State, i: u8) {
    let items = Box::new([("back", Scene::Boards), ("exit", Scene::List)]);
    state.button_group = Some(ButtonGroup::new(items));

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
    let my_name = get_name_buf(get_me());
    for score in raw_scores.me.iter() {
        let score = *score;
        if !valid_score(board, score) {
            continue;
        }
        let value: String = format_score(board, score);
        scores.push(ScoreInfo {
            name: my_name.clone(),
            value,
            raw_value: score,
            me: true,
        });
    }
    let friend_names = load_friend_names();
    let default_name = "anonymous";
    for friend in raw_scores.friends.iter() {
        let score = friend.score;
        if !valid_score(board, score) {
            continue;
        }
        let value: String = format_score(board, score);
        let friend_name = friend_names.get(usize::from(friend.index));
        let friend_name = match friend_name {
            Some(friend_name) => friend_name.clone(),
            None => default_name.to_string(),
        };
        scores.push(ScoreInfo {
            name: friend_name,
            value,
            raw_value: score,
            me: false,
        });
    }
    bubble_sort(&mut scores);
    app.scores = Some(scores);
}

pub fn update(state: &mut State) {
    if let Some(button_group) = state.button_group.as_mut() {
        if let Some(scene) = button_group.update(&state.input) {
            state.transition_to(scene);
        }
    }
}

pub fn render(state: &State) {
    clear_screen(Color::White);
    let font = state.font.as_font();
    let app = &state.apps[state.pos];
    let Some(scores) = &app.scores else {
        return;
    };
    let mut i = 0;
    for score in scores.iter().take(8) {
        i += 1;
        let color = if score.me { Color::Green } else { Color::Black };
        let point = Point::new(6, LINE_HEIGHT * i);
        draw_text(&score.name, &font, point, color);
        let point = Point::new(140, LINE_HEIGHT * i);
        draw_text(&score.value, &font, point, color);
    }
    if let Some(button_group) = &state.button_group {
        button_group.render(&font);
    }
}

fn load_friend_names() -> Vec<String> {
    let Some(raw) = sudo::load_file_buf("friends") else {
        return Vec::new();
    };
    let raw = raw.into_vec();
    let raw = unsafe { String::from_utf8_unchecked(raw) };
    let mut names = Vec::new();
    for name in raw.split('\n') {
        names.push(name.to_string());
    }
    names
}

const fn valid_score(board: &BoardInfo, score: i16) -> bool {
    score != 0 && score >= board.min && score <= board.max
}

fn format_score(board: &BoardInfo, score: i16) -> String {
    let val = score.unsigned_abs();
    if board.time {
        format_time(val)
    } else if board.decimals > 0 {
        format_decimal(val, board.decimals)
    } else {
        val.to_string()
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
