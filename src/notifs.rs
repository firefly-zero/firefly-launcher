use firefly_sudo::sudo;

use crate::apps::App;

#[derive(Default)]
pub struct Notif {
    pub manual: bool,
    pub badges: bool,
    pub boards: bool,
}

impl Notif {
    pub fn new(app: &App) -> Self {
        let goals = Goal::new(app);
        Self {
            manual: goals.manual_size != 0,
            badges: goals.new_badges != 0,
            boards: false,
        }
    }
}

pub struct Goal {
    manual_size: u32,
    new_badges: u8,
    boards_sum: u64,
}

impl Goal {
    fn new(app: &App) -> Self {
        let mut new_badges: u8 = 0;
        let mut boards_sum: u64 = 0;
        if let Some(stats) = &app.stats {
            for badge in &stats.badges {
                if badge.new {
                    new_badges = new_badges.saturating_add(1);
                }
            }
            for board in &stats.scores {
                boards_sum += u64::from(board.me[0].unsigned_abs());
                boards_sum += u64::from(board.friends[0].score.unsigned_abs());
            }
        }
        let manual_path = alloc::format!("roms/{}/{}/_manual", app.author_id, app.id);
        let manual_size = sudo::get_file_size(&manual_path) as u32;
        Self {
            manual_size,
            new_badges,
            boards_sum,
        }
    }
}
