use crate::apps::App;
use firefly_sudo::sudo;
use serde::*;

#[derive(Default, Serialize, Deserialize)]
pub struct NamedNotif<'a> {
    pub author: &'a str,
    pub app: &'a str,
    pub notif: Notif,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Notif {
    pub manual: bool,
    pub badges: bool,
    pub boards: bool,

    manual_size: u32,
    my_boards: u64,
    friends_boards: u64,
}

impl Notif {
    pub fn load_into(app: &mut App) {
        app.try_load_stats();
        let new = Self::new(app);
        let notif = match &app.notif {
            Some(old) => new.merge(old),
            None => new,
        };
        app.notif = Some(notif);
    }

    pub fn new(app: &App) -> Self {
        let mut new_badges = false;
        let mut my_boards: u64 = 0;
        let mut friends_boards: u64 = 0;
        if let Some(stats) = &app.stats {
            for badge in &stats.badges {
                if badge.new {
                    new_badges = true;
                    break;
                }
            }
            for board in &stats.scores {
                my_boards += u64::from(board.me[0].unsigned_abs());
                friends_boards += u64::from(board.friends[0].score.unsigned_abs());
            }
        }
        let manual_path = alloc::format!("roms/{}/{}/_manual", app.author_id, app.id);
        let manual_size = sudo::get_file_size(&manual_path) as u32;

        Self {
            manual: manual_size != 0 && !new_badges,
            badges: new_badges,
            boards: false,
            manual_size,
            my_boards,
            friends_boards,
        }
    }

    #[must_use]
    const fn merge(&self, old: &Self) -> Self {
        Self {
            manual: self.manual || (self.manual_size != old.manual_size),
            badges: self.badges,
            boards: self.boards
                || (self.my_boards != old.my_boards)
                || (self.friends_boards != old.friends_boards),
            manual_size: self.manual_size,
            my_boards: self.my_boards,
            friends_boards: self.friends_boards,
        }
    }
}
