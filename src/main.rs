#![no_std]
#![no_main]
#![deny(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::similar_names)]

mod apps;
mod list;
mod state;
use apps::*;
use list::*;
use state::*;

extern crate alloc;

use firefly_rust::*;
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

#[no_mangle]
extern fn handle_menu(i: u32) {
    assert!(i == 0);
}

#[no_mangle]
extern fn boot() {
    add_menu_item(0, "app info");
    init_state()
}

#[no_mangle]
extern fn update() {
    update_list();
}

#[no_mangle]
extern fn render() {
    render_list();
}
