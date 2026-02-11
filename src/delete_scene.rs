use crate::*;
use alloc::format;
use firefly_rust::*;

pub const fn init(state: &mut State) {
    state.dialog_yes = false;
}

pub fn update(state: &mut State) {
    let input = state.input.get();
    match input {
        Input::Select => {
            if state.dialog_yes {
                delete_app(state);
                state.transition_to(Scene::List);
            } else {
                state.transition_to(Scene::Info);
            }
        }
        Input::Right => {
            state.dialog_yes = true;
        }
        Input::Left => {
            state.dialog_yes = false;
        }
        _ => (),
    }
}

fn delete_app(state: &mut State) {
    let app = &mut state.apps[state.pos];
    app.data_size = None;
    let data_path = format!("data/{}/{}/etc", app.author_id, app.id);
    let files = sudo::DirBuf::list_dirs(&data_path);
    for file_path in files.iter() {
        // TODO: actually remove files when runtime supports it
        let msg = format!("cannot remove {file_path}: file removal not implemented yet");
        log_error(&msg);
    }
}

pub fn render(state: &State) {
    const MARGIN: i32 = 46;

    let theme = state.settings.theme;
    clear_screen(theme.bg);
    let app = &state.apps[state.pos];

    let font = state.font.as_font();
    let corner = Size::new(4, 4);
    let box_style = Style::outlined(theme.primary, 1);

    {
        let text = format!(
            "Wanna delete data for\n{}?\n\nThere's no going back.",
            app.name
        );
        let point = Point::new(MARGIN + 6, MARGIN + 16);
        draw_text(&text, &font, point, theme.primary);
        draw_rounded_rect(
            Point::new(MARGIN, MARGIN),
            Size::new(WIDTH - MARGIN * 2, HEIGHT - MARGIN * 2),
            corner,
            box_style,
        );
    }

    let box_width = WIDTH - MARGIN * 2;
    let btn_width = 24;
    let y = 96;
    {
        let x = MARGIN + box_width / 2 - (btn_width + btn_width / 2);
        let point = Point::new(x + 3, y + 7);
        draw_text("nuh", &font, point, theme.primary);
        if !state.dialog_yes {
            draw_rounded_rect(
                Point::new(x, y),
                Size::new(btn_width, 12),
                corner,
                box_style,
            );
        }
    }

    {
        let x = MARGIN + box_width / 2 + btn_width / 2;
        let point = Point::new(x + 3, y + 7);
        draw_text("yep", &font, point, theme.primary);
        if state.dialog_yes {
            draw_rounded_rect(
                Point::new(x, y),
                Size::new(btn_width, 12),
                corner,
                box_style,
            );
        }
    }
}
