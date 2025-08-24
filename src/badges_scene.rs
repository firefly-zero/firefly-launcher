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
}

pub fn init(state: &mut State) {
    state.old_buttons = Buttons::default();
    let items = Box::new([("back", Scene::Info), ("exit", Scene::List)]);
    state.button_group = Some(ButtonGroup::new(items));
}

pub fn update(state: &mut State) {
    let app = &mut state.apps[state.pos];
    if app.badges.is_none() {
        let badges_path = format!("roms/{}/{}/_badges", app.author_id, app.id);
        // TODO: don't unwrap
        let raw = sudo::load_file_buf(&badges_path).unwrap();
        let raw_badges = firefly_types::Badges::decode(raw.data()).unwrap();
        let mut badges = Vec::with_capacity(raw_badges.badges.len());
        for badge in raw_badges.badges.iter() {
            badges.push(BadgeInfo {
                name: badge.name.to_owned(),
            });
        }
        app.badges = Some(badges);
    }
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
        // for (badge, i) in badges.iter().zip(0..) {
        //     render_badge(&font, i, badge);
        // }
    }
    if let Some(button_group) = &state.button_group {
        button_group.render(&font);
    }
}

fn render_badge(font: &Font<'_>, i: i32, b: &BadgeInfo) {
    let point = Point::new(100, LINE_HEIGHT * i);
    draw_text(&b.name, font, point, Color::Blue);
}
