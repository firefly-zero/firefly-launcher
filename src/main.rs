#![no_std]
#![no_main]
extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::{Cell, OnceCell};
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
static mut POS: usize = 0;
static mut OLD_BUTTONS: Cell<Option<Buttons>> = Cell::new(None);
static mut COMMAND: Cell<Option<Command>> = Cell::new(None);

pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 160;

enum Command {
    GoDown,
    GoUp,
    Launch,
}

struct App {
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
                app_name:  meta.app_name.to_string(),
                author_id: meta.author_id.to_string(),
                app_id:    meta.app_id.to_string(),
            });
        }
    }
    unsafe { APPS.set(apps) }.ok().unwrap();
}

#[no_mangle]
extern fn update() {
    handle_input()
}

#[no_mangle]
extern fn render() {
    clear_screen(Color::LIGHT);
    let font = unsafe { FONT.get() }.unwrap();
    let font: Font = font.into();
    let apps = unsafe { APPS.get() }.unwrap();
    apply_command();
    draw_selection();
    for (i, app) in apps.iter().enumerate() {
        draw_text(
            &app.app_name,
            &font,
            Point {
                x: 20,
                y: 10 + i as i32 * 10,
            },
            Color::DARK,
        );
    }
}

fn handle_input() {
    let new_buttons = read_buttons();
    let old_buttons = unsafe { OLD_BUTTONS.get_mut() };
    let Some(old_buttons) = old_buttons else {
        let old_buttons = Some(Buttons::default());
        unsafe {
            OLD_BUTTONS.set(old_buttons);
        }
        return;
    };
    let pressed = new_buttons.just_pressed(old_buttons);
    unsafe {
        OLD_BUTTONS.set(Some(new_buttons));
    };
    let command = if pressed.x {
        Some(Command::GoDown)
    } else if pressed.y {
        Some(Command::GoUp)
    } else if pressed.a {
        Some(Command::Launch)
    } else {
        None
    };
    unsafe {
        COMMAND.set(command);
    }
}

fn draw_selection() {
    let pos = unsafe { POS };
    draw_circle(
        Point {
            x: 10,
            y: 4 + pos as i32 * 10,
        },
        8,
        Style {
            fill_color: Color::DARK,
            ..Style::default()
        },
    );
}

fn apply_command() {
    let command = unsafe { COMMAND.take() };
    let Some(command) = command else { return };
    match command {
        Command::GoDown => unsafe { POS += 1 },
        Command::GoUp => unsafe {
            POS = POS.saturating_sub(1);
        },
        Command::Launch => todo!(),
    }
}
