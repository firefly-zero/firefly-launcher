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
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::wildcard_imports,
    clippy::similar_names,
    clippy::module_name_repetitions
)]

extern crate alloc;

mod apps;
mod delete_scene;
mod info_scene;
mod list_scene;
mod scroll;
mod state;

use apps::*;
use firefly_rust::*;
use list_scene::Command;
use state::*;

/// Frame number to track the loading progress.
static mut FRAME: u8 = 0;
/// If the state is not initialized yet.
static mut LOADING: bool = true;

pub enum Scene {
    List,
    Info,
    ClearData,
}

#[no_mangle]
extern "C" fn handle_menu(i: u32) {
    let state = get_state();
    if i == 1 {
        state.transition_to(Scene::Info);
    }
}

#[no_mangle]
extern "C" fn boot() {
    let splash = load_file_buf("_splash").unwrap();
    let splash = splash.as_image();
    draw_image(&splash, Point::MIN);
    add_menu_item(1, "app info");
}

#[no_mangle]
extern "C" fn update() {
    let frame = unsafe { FRAME };
    // On the first few updates, do nothing,
    // let "render" to render the splash screen.
    if frame <= 1 {
        unsafe { FRAME += 1 };
        return;
    }
    // After that, load the list of apps.
    if frame == 2 {
        unsafe { FRAME += 1 };
        init_state();
        unsafe { LOADING = false };
        return;
    }

    let state = get_state();
    match state.scene() {
        Scene::List => list_scene::update(state),
        Scene::Info => info_scene::update(state),
        Scene::ClearData => delete_scene::update(state),
    }
}

#[no_mangle]
extern "C" fn render() {
    let loading = unsafe { LOADING };
    if loading {
        return;
    }

    let state = get_state();
    match state.scene() {
        Scene::List => list_scene::render(state),
        Scene::Info => info_scene::render(state),
        Scene::ClearData => delete_scene::render(state),
    }
}

#[no_mangle]
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
