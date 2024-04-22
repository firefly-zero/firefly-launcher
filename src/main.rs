#![no_std]
#![no_main]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_meta::Meta;
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
static mut APPS: OnceCell<Vec<App>> = OnceCell::new();

pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 160;

struct App {
    path:      String,
    app_name:  String,
    author_id: String,
    app_id:    String,
}

#[no_mangle]
extern fn boot() {
    let font = rom::load_buf("font");
    unsafe { FONT.set(font) }.ok().unwrap();

    let author_dirs = sudo::DirBuf::list_dirs("roms");
    let mut apps: Vec<App> = Vec::new();
    for author_dir in author_dirs.iter() {
        let author_path = format!("roms/{author_dir}");
        let app_dirs = sudo::DirBuf::list_dirs(&author_path);
        for app_dir in app_dirs.iter() {
            let app_path = format!("{author_path}/{app_dir}");
            let meta_path = format!("{app_path}/meta");
            let meta_raw = sudo::load_file_buf(&meta_path);
            let Ok(meta) = Meta::decode(meta_raw.data()) else {
                continue;
            };
            apps.push(App {
                path:      app_path,
                app_name:  meta.app_name.to_string(),
                author_id: meta.author_id.to_string(),
                app_id:    meta.app_id.to_string(),
            });
        }
    }
    unsafe { APPS.set(apps) }.ok().unwrap();
}

#[no_mangle]
extern fn render() {
    clear_screen(Color::LIGHT);
    let font = unsafe { FONT.get() }.unwrap();
    let font: Font = font.into();
    let apps = unsafe { APPS.get() }.unwrap();
    for (i, app) in apps.iter().enumerate() {
        draw_text(
            &app.app_name,
            &font,
            Point {
                x: 10,
                y: 10 + i as i32 * 10,
            },
            Color::DARK,
        );
    }
}
