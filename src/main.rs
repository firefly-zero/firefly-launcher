#![no_std]
#![no_main]
use core::cell::OnceCell;
use firefly_rust::*;
use talc::locking::AssumeUnlockable;
use talc::{ClaimOnOom, Span, Talc, Talck};

// one wasm page
static mut ARENA: [u8; 65536] = [0; 65536];

#[global_allocator]
static ALLOCATOR: Talck<AssumeUnlockable, ClaimOnOom> = Talc::new(unsafe {
    //
    ClaimOnOom::new(Span::from_const_array(core::ptr::addr_of!(ARENA)))
})
.lock();

static mut FONT: OnceCell<FileBuf> = OnceCell::new();

pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 160;

#[no_mangle]
extern fn boot() {
    let font = FileBuf::load("font");
    unsafe { FONT.set(font) }.ok().unwrap();
}

#[no_mangle]
extern fn render() {
    clear_screen(Color::LIGHT);
    let font = unsafe { FONT.get() }.unwrap();
    let font: Font = font.into();
    draw_text("Hello", font, Point { x: 10, y: 10 }, Color::DARK);
}
