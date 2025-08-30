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
    pub rom_size: Option<usize>,
    pub data_size: Option<usize>,
    pub stats: Option<Stats>,
    pub badges: Option<Vec<BadgeInfo>>,
    pub boards: Option<Vec<BoardInfo>>,
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
pub fn read_apps() -> Vec<App> {
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
            let id = (meta.author_id, meta.app_id);
            if id == ("sys", "launcher") || id == ("sys", "connector") {
                continue;
            }
            apps.push(App {
                id: meta.app_id.to_string(),
                author_id: meta.author_id.to_string(),
                name: meta.app_name.to_string(),
                author_name: meta.author_name.to_string(),
                rom_size: None,
                data_size: None,
                stats: None,
                badges: None,
                boards: None,
            });
        }
    }
    bubble_sort(&mut apps);
    apps
}

/// Good old bubble sort. Slower but much smaller than the built-in sort function.
fn bubble_sort(apps: &mut [App]) {
    let len = apps.len();
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..len - 1 {
            if ascii_gt(&apps[i].name, &apps[i + 1].name) {
                apps.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

/// Case-insensitive comparison of two ASCII strings.
fn ascii_gt(s1: &str, s2: &str) -> bool {
    for (c1, c2) in s1.as_bytes().iter().zip(s2.as_bytes()) {
        let c1 = c1.to_ascii_lowercase();
        let c2 = c2.to_ascii_lowercase();
        if c1 != c2 {
            return c1 > c2;
        }
    }
    s1.len() > s2.len()
}
