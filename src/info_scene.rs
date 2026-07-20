use crate::*;
use alloc::format;
use alloc::vec::Vec;
use firefly_rust::*;
use firefly_sudo::sudo;

const LINE_HEIGHT: i32 = 12;
const CATALOG_URL: &str = "https://catalog.fireflyzero.com/";

static FIELDS: &[Message] = &[
    Message::AppID,
    Message::AuthorName,
    Message::AppName,
    Message::Size,
    Message::Launches,
    Message::Installed,
    Message::Updated,
];

pub fn init(state: &mut State) {
    let app = &mut state.apps[state.pos];
    if app.size.is_none() {
        let app_path = format!("roms/{}/{}", app.author_id, app.id);
        let rom_size = get_dir_size(&app_path);
        let data_path = format!("data/{}/{}/etc", app.author_id, app.id);
        let data_size = get_dir_size(&data_path);
        app.size = Some((rom_size, data_size));
    }
    app.try_load_stats();
    Notif::load_into(app);
    let notif = app.notif.as_ref().unwrap();

    let lang = state.settings.language;
    let mut items = Vec::new();
    if let Some(stats) = &app.stats {
        if !stats.badges.is_empty() {
            items.push(Button {
                text: Message::Achievements.translate(lang),
                scene: Scene::Delegate("sys", "badges"),
                accent: notif.badges,
            });
        }
        if !stats.scores.is_empty() {
            let msg = Message::Scoreboards.translate(lang);
            items.push(Button {
                text: msg,
                scene: Scene::Delegate("sys", "boards"),
                accent: notif.boards,
            });
        }
    }
    let manual_path = alloc::format!("roms/{}/{}/_manual", app.author_id, app.id);
    let has_manual = sudo::get_file_size(&manual_path) != 0;
    if has_manual {
        let msg = Message::Manual.translate(lang);
        items.push(Button {
            text: msg,
            scene: Scene::Delegate("sys", "manuals"),
            accent: notif.manual,
        });
    }
    if can_delete(app) {
        let msg = Message::Remove.translate(lang);
        items.push(Button {
            text: msg,
            scene: Scene::Delegate("sys", "remover"),
            accent: false,
        });
    }
    state.button_group = Some(ButtonGroup::new(items.into_boxed_slice()));
}

fn can_delete(app: &App) -> bool {
    if app.author_id == "sys" {
        return app.id != "installer" && app.id != "connector";
    }
    true
}

pub fn update(state: &mut State) {
    if let Some(button_group) = state.button_group.as_mut() {
        if let Some(scene) = button_group.update(&state.input) {
            state.transition_to(scene);
        }
    }
}

fn get_dir_size(dir_path: &str) -> usize {
    let files = sudo::DirBuf::list_files(dir_path);
    let mut size = 0;
    for file in files.iter() {
        let file_path = format!("{dir_path}/{file}");
        size += sudo::get_file_size(&file_path);
    }
    size
}

pub fn render(state: &State) {
    clear_screen(state.settings.theme.bg);
    let font = &state.font;
    for (message, i) in FIELDS.iter().zip(1..) {
        let point = Point::new(6, LINE_HEIGHT * i);
        let text = message.translate(state.settings.language);
        draw_text(text, font, point, state.settings.theme.primary);
    }
    let theme = state.settings.theme;
    let app = &state.apps[state.pos];
    let color = if state.settings.contrast {
        theme.primary
    } else {
        theme.accent
    };
    let full_id = alloc::format!("{}.{}", app.author_id, app.id);
    render_info(font, color, 1, &full_id);
    render_info(font, color, 2, &app.author_name);
    render_info(font, color, 3, &app.name);
    if let Some((app_size, data_size)) = app.size {
        let app_size = format_size(app_size);
        let size = if data_size == 0 {
            app_size
        } else {
            let data_size = format_size(data_size);
            alloc::format!("{app_size} + {data_size}")
        };
        render_info(font, color, 4, &size);
    }
    if let Some(stats) = &app.stats {
        let single = stats.launches[0];
        let multi = stats.launches[1] + stats.launches[2] + stats.launches[3];
        let launches = if multi == 0 {
            format!("{single}")
        } else {
            format!("{single} + {multi}")
        };
        render_info(font, color, 5, &launches);
        let installed_on = format_date(stats.installed_on);
        render_info(font, color, 6, &installed_on);
        let updated_on = format_date(stats.updated_on);
        render_info(font, color, 7, &updated_on);
    }

    if let Some(button_group) = &state.button_group {
        button_group.render(font, &state.settings.theme);
    }

    render_qr(theme, app);
}

fn render_qr(theme: Theme, app: &App) {
    let url = alloc::format!("{CATALOG_URL}{}.{}", app.author_id, app.id);
    let w = if url.len() <= 42 { 37 } else { 41 };
    let point = Point::new((WIDTH - w) - 10, HEIGHT - w - 6);
    let size = Size::new(w + 2, w + 2);
    let style = Style::solid(theme.primary);
    draw_rect(Point::new(point.x - 1, point.y - 1), size, style);
    draw_rect(Point::new(point.x, point.y), size, style);
    draw_qr(&url, point, Color::Black, Color::White);
}

fn render_info(font: &FontBuf, color: Color, i: i32, t: &str) {
    let point = Point::new(100, LINE_HEIGHT * i);
    draw_text(t, font, point, color);
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
