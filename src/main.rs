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
mod state;

use apps::*;
use core::ptr::addr_of;
use firefly_rust::add_menu_item;
use list_scene::Command;
use state::*;
use talc::{locking::AssumeUnlockable, ClaimOnOom, Span, Talc, Talck};

// one wasm page
static mut ARENA: [u8; 65536] = [0; 65536];

#[global_allocator]
static ALLOCATOR: Talck<AssumeUnlockable, ClaimOnOom> = Talc::new(unsafe {
    //
    ClaimOnOom::new(Span::from_array(addr_of!(ARENA).cast_mut()))
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
