use crate::*;
use alloc::format;
use firefly_rust::*;

const LINE_HEIGHT: i32 = 12;

static FIELDS: &[&str] = &[
    "Author ID:",
    "App ID:",
    "Author name:",
    "App name:",
    "ROM size:",
    "Data size:",
];

pub fn init(state: &mut State) {
    state.old_buttons = Buttons::default()
}

pub fn update(state: &mut State) {
    let new_buttons = read_buttons(Player::P0);
    let released_buttons = new_buttons.just_released(&state.old_buttons);
    if released_buttons.any() {
        state.transition_to(Scene::List);
        // open_menu();
    }
    state.old_buttons = new_buttons;

    let app = &mut state.apps[state.pos];
    if app.rom_size.is_none() {
        let app_path = format!("roms/{}/{}", app.author_id, app.id);
        app.rom_size = Some(get_dir_size(&app_path));
    }
    if app.data_size.is_none() {
        let data_path = format!("data/{}/{}", app.author_id, app.id);
        app.data_size = Some(get_dir_size(&data_path));
    }
}

fn get_dir_size(dir_path: &str) -> usize {
    let files = sudo::DirBuf::list_dirs(&dir_path);
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
}

fn render_info(font: &Font, i: i32, t: &str) {
    let point = Point::new(100, LINE_HEIGHT * i);
    draw_text(t, &font, point, Color::DarkBlue);
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
