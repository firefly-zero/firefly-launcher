#![no_std]
#![no_main]
#![deny(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]

mod apps;
mod delete_scene;
mod info_scene;
mod list_scene;
mod state;

use apps::*;
use list_scene::Command;
use state::*;

extern crate alloc;

use firefly_rust::add_menu_item;
use talc::locking::AssumeUnlockable;
use talc::{ClaimOnOom, Span, Talc, Talck};

// one wasm page
static mut ARENA: [u8; 65536] = [0; 65536];

// Setup Talc as the global allocator.
#[global_allocator]
static ALLOCATOR: Talck<AssumeUnlockable, ClaimOnOom> = Talc::new(unsafe {
    //
    ClaimOnOom::new(Span::from_const_array(core::ptr::addr_of!(ARENA)))
})
.lock();

pub enum Scene {
    List,
    Info,
    ClearData,
}

#[no_mangle]
extern fn handle_menu(i: u32) {
    let state = get_state();
    match i {
        1 => state.transition_to(Scene::Info),
        2 => state.transition_to(Scene::ClearData),
        _ => unreachable!(),
    }
}

#[no_mangle]
extern fn boot() {
    add_menu_item(1, "app info");
    add_menu_item(2, "clear data");
    init_state();
}

#[no_mangle]
extern fn update() {
    let state = get_state();
    match state.scene() {
        Scene::List => list_scene::update(state),
        Scene::Info => info_scene::update(state),
        Scene::ClearData => delete_scene::update(state),
    }
}

#[no_mangle]
extern fn render() {
    let state = get_state();
    match state.scene() {
        Scene::List => list_scene::render(state),
        Scene::Info => info_scene::render(state),
        Scene::ClearData => delete_scene::render(state),
    }
}
