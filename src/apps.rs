use crate::*;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use firefly_rust::*;
use firefly_types::{Encode, Meta, Stats};

pub struct App {
    pub id: String,
    pub author_id: String,
    pub name: String,
    pub author_name: String,
    pub priority: u8,

    pub rom_size: Option<usize>,
    pub data_size: Option<usize>,
    pub stats: Option<Stats>,
    pub badges: Option<Vec<BadgeInfo>>,
    pub boards: Option<Vec<BoardInfo>>,
    pub scores: Option<Vec<ScoreInfo>>,
}

impl Gt for App {
    fn gt(&self, other: &Self) -> bool {
        if self.priority != other.priority {
            return self.priority > other.priority;
        }
        ascii_gt(&self.name, &other.name)
    }
}

impl App {
    pub fn try_load_stats(&mut self) {
        if self.stats.is_some() {
            return;
        }
        let stats_path = format!("data/{}/{}/stats", self.author_id, self.id);
        let Some(raw) = sudo::load_file_buf(&stats_path) else {
            return;
        };
        let Ok(stats) = firefly_types::Stats::decode(raw.data()) else {
            return;
        };
        self.stats = Some(stats);
    }
}

/// Go through all ROMs and read their metadata.
pub fn read_apps(is_online: bool) -> Vec<App> {
    let author_dirs = sudo::DirBuf::list_dirs("roms");
    let mut apps: Vec<App> = Vec::new();
    for author_dir in author_dirs.iter() {
        let author_path = format!("roms/{author_dir}");
        let app_dirs = sudo::DirBuf::list_dirs(&author_path);
        for app_dir in app_dirs.iter() {
            let app_path = format!("{author_path}/{app_dir}");
            let meta_path = format!("{app_path}/_meta");
            let meta_raw = if let Some(meta_raw) = sudo::load_file_buf(&meta_path) {
                meta_raw
            } else {
                let meta_path = format!("{app_path}/meta");
                let Some(meta_raw) = sudo::load_file_buf(&meta_path) else {
                    continue;
                };
                meta_raw
            };
            let Ok(meta) = Meta::decode(meta_raw.data()) else {
                continue;
            };
            // Hide the launcher itself from the list.
            let mut app_id = meta.app_id;
            let mut app_name = meta.app_name;
            let mut priority = 10;
            if meta.author_id == "sys" {
                priority = 5;
                if meta.app_id == "launcher" {
                    app_name = "Refresh app list";
                }
                if meta.app_id == "connector" {
                    priority = 1;
                    if is_online {
                        app_id = "disconnector";
                        app_name = "Disconnect";
                    } else {
                        app_name = "Start multiplayer";
                    }
                }
            }
            apps.push(App {
                id: app_id.to_string(),
                author_id: meta.author_id.to_string(),
                name: app_name.to_string(),
                author_name: meta.author_name.to_string(),
                priority,
                rom_size: None,
                data_size: None,
                stats: None,
                badges: None,
                boards: None,
                scores: None,
            });
        }
    }
    bubble_sort(&mut apps);
    apps
}
