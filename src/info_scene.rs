use crate::*;
use alloc::{boxed::Box, format};
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;

static FIELDS: &[&str] = &[
    "author ID",
    "app ID",
    "author name",
    "app name",
    "ROM size",
    "data size",
];

pub fn init(state: &mut State) {
    state.old_buttons = Buttons::default();
    let items = Box::new([
        ("stats", Scene::Stats),
        ("clear data", Scene::ClearData),
        ("back", Scene::List),
    ]);
    state.button_group = Some(ButtonGroup::new(items));
}

pub fn update(state: &mut State) {
    let app = &mut state.apps[state.pos];
    if app.rom_size.is_none() {
        let app_path = format!("roms/{}/{}", app.author_id, app.id);
        app.rom_size = Some(get_dir_size(&app_path));
    }
    if app.data_size.is_none() {
        let data_path = format!("data/{}/{}/etc", app.author_id, app.id);
        app.data_size = Some(get_dir_size(&data_path));
    }
    if let Some(button_group) = state.button_group.as_mut() {
        if let Some(scene) = button_group.update() {
            state.transition_to(scene);
        }
    }
}

fn get_dir_size(dir_path: &str) -> usize {
    let files = sudo::DirBuf::list_dirs(dir_path);
    let mut size = 0;
    for file in files.iter() {
        let file_path = format!("{dir_path}/{file}");
        size += sudo::get_file_size(&file_path);
    }
    size
}

pub fn render(state: &State) {
    clear_screen(Color::White);
    let font = state.font.as_font();
    for (text, i) in FIELDS.iter().zip(1..) {
        let point = Point::new(6, LINE_HEIGHT * i);
        draw_text(text, &font, point, Color::DarkBlue);
    }
    let app = &state.apps[state.pos];
    render_info(&font, 1, &app.author_id);
    render_info(&font, 2, &app.id);
    render_info(&font, 3, &app.author_name);
    render_info(&font, 4, &app.name);
    if let Some(size) = app.rom_size {
        render_info(&font, 5, &format_size(size));
    }
    if let Some(size) = app.data_size {
        render_info(&font, 6, &format_size(size));
    }
    if let Some(button_group) = &state.button_group {
        button_group.render(&font);
    }
}

fn render_info(font: &Font<'_>, i: i32, t: &str) {
    let point = Point::new(100, LINE_HEIGHT * i);
    draw_text(t, font, point, Color::Blue);
}

fn format_size(size: usize) -> alloc::string::String {
    if size > 2 * 1024 * 1024 {
        let size = size / (1024 * 1024);
        return format!("{size} MB");
    }
    if size > 2048 {
        let size = size / 1024;
        return format!("{size} KB");
    }
    format!("{size} B")
}
