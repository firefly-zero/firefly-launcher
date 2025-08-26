use crate::*;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{boxed::Box, format};
use firefly_rust::*;
use firefly_types::Encode;

const LINE_HEIGHT: i32 = 12;

pub struct BadgeInfo {
    name: String,
    done: u16,
    goal: u16,
}

pub fn init(state: &mut State) {
    state.old_buttons = Buttons::default();
    let items = Box::new([("back", Scene::Info), ("exit", Scene::List)]);
    state.button_group = Some(ButtonGroup::new(items));

    let app = &mut state.apps[state.pos];
    app.try_load_stats();
    try_load_badges(app);
}

fn try_load_badges(app: &mut App) {
    let Some(stats) = app.stats.as_ref() else {
        return;
    };
    if app.badges.is_some() {
        return;
    }
    let badges_path = format!("roms/{}/{}/_badges", app.author_id, app.id);
    let Some(raw) = sudo::load_file_buf(&badges_path) else {
        return;
    };
    let Ok(raw_badges) = firefly_types::Badges::decode(raw.data()) else {
        return;
    };
    let mut badges = Vec::with_capacity(raw_badges.badges.len());
    for (info, progress) in raw_badges.badges.iter().zip(&stats.badges) {
        if progress.done < info.hidden {
            continue;
        }
        badges.push(BadgeInfo {
            name: info.name.to_owned(),
            done: progress.done,
            goal: progress.goal,
        });
    }
    // TODO: sort badges
    app.badges = Some(badges);
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
    if let Some(badges) = &app.badges {
        // for (badge, i) in stats.badges.iter().zip(0..) {
        for (badge, i) in badges.iter().zip(1..) {
            render_badge(&font, i, badge);
        }
    }
    if let Some(button_group) = &state.button_group {
        button_group.render(&font);
    }
}

fn render_badge(font: &Font<'_>, i: i32, b: &BadgeInfo) {
    let point = Point::new(6, LINE_HEIGHT * i);
    let color = if b.done >= b.goal {
        Color::DarkBlue
    } else if b.done > 0 {
        Color::Gray
    } else {
        Color::LightGray
    };
    draw_text(&b.name, font, point, color);

    let point = Point::new(100, point.y - LINE_HEIGHT + 4);
    let style = Style {
        fill_color: Color::None,
        stroke_color: color,
        stroke_width: 1,
    };
    let size = Size::new(WIDTH - 106, LINE_HEIGHT - 2);
    draw_rect(point, size, style);

    let style = Style {
        fill_color: color,
        stroke_color: Color::None,
        stroke_width: 0,
    };
    let ratio = (f32::from(b.done) / f32::from(b.goal)).clamp(0., 1.);
    #[expect(clippy::cast_precision_loss)]
    let width = (size.width as f32 * ratio) as i32;
    let size = Size::new(width, size.height);
    draw_rect(point, size, style);
}
