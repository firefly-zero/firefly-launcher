#![no_std]
#![no_main]
#![deny(
    rust_2018_idioms,
    redundant_lifetimes,
    redundant_semicolons,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::allow_attributes
)]
#![expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::collapsible_if,
    clippy::option_if_let_else,
    clippy::wildcard_imports
)]

extern crate alloc;

mod apps;
mod badges_scene;
mod boards_scene;
mod button_group;
mod catalog_scene;
mod components;
mod delete_scene;
mod info_scene;
mod input;
mod list_scene;
mod scores_scene;
mod scroll;
mod state;
mod stats_scene;
mod translations;
mod utils;

use apps::*;
use badges_scene::BadgeInfo;
use boards_scene::BoardInfo;
use button_group::ButtonGroup;
use components::*;
use firefly_rust::*;
use input::{Input, InputManager};
use scores_scene::ScoreInfo;
use state::*;
use translations::Message;
use utils::*;

/// If the splash screen was rendered on the screen.
static mut RENDERED: bool = false;
/// If the state is not initialized yet.
static mut LOADING: bool = true;

#[derive(Clone, Copy)]
pub enum Scene {
    List,
    Info,
    Stats,
    Badges,
    Boards,
    Catalog,
    Scores(u8),
    ClearData,
}

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let splash = load_file_buf("_splash").unwrap();
    let splash = splash.as_image();
    draw_image(&splash, Point::MIN);
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    // Wait for the splash screen to be rendered.
    if !unsafe { RENDERED } {
        return;
    }
    if unsafe { LOADING } {
        init_state();
        unsafe { LOADING = false };
        return;
    }

    let state = get_state();
    state.input.update();
    match state.scene() {
        Scene::List => list_scene::update(state),
        Scene::Info => info_scene::update(state),
        Scene::Stats => stats_scene::update(state),
        Scene::Badges => badges_scene::update(state),
        Scene::Boards => boards_scene::update(state),
        Scene::Scores(_) => scores_scene::update(state),
        Scene::Catalog => catalog_scene::update(state),
        Scene::ClearData => delete_scene::update(state),
    }
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    // During the first rendering iteration,
    // we don't render anything from the "render" callback.
    // Instead, we let the runtime to display the splash screen
    // rendered earlier from "boot".
    if !unsafe { RENDERED } {
        unsafe { RENDERED = true };
        return;
    }
    // Don't render until the list of apps is loaded.
    if unsafe { LOADING } {
        return;
    }

    let state = get_state();
    // TODO(@orsinium): enable when we have support for menu_closed callback.
    // When menu is opened and closed, we have to mark input as dirty
    // to re-render the frame.
    //
    // if !state.input.dirty {
    //     return;
    // }
    state.input.dirty = false;
    match state.scene() {
        Scene::List => list_scene::render(state),
        Scene::Info => info_scene::render(state),
        Scene::Stats => stats_scene::render(state),
        Scene::Badges => badges_scene::render(state),
        Scene::Boards => boards_scene::render(state),
        Scene::Scores(_) => scores_scene::render(state),
        Scene::Catalog => catalog_scene::render(state),
        Scene::ClearData => delete_scene::render(state),
    }
}

#[unsafe(no_mangle)]
extern "C" fn cheat(cmd: i32, val: i32) -> i32 {
    if cmd == 1 {
        let state = get_state();
        let Ok(index) = usize::try_from(val) else {
            return 0;
        };
        if let Some(app) = state.apps.get(index) {
            sudo::run_app(&app.author_id, &app.id);
            return 1;
        }
        return 0;
    }
    0
}
