use crate::*;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use firefly_rust::*;
use firefly_types::{Encode, Meta, Stats};

pub struct App {
    /// The first part of the full app ID.
    pub author_id: String,
    /// The second part of the full app ID.
    pub id: String,
    /// The human-readable app name. Shown in the list of apps.
    pub name: String,
    /// The human-readable author name. Shown in the list of apps.
    pub author_name: String,
    /// The app order on the screen.
    ///
    /// First we show system apps, then favorites, and then the rest.
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
    let raw_vec = read_metas();
    let mut raw_slice = &raw_vec[..];
    let mut apps: Vec<App> = Vec::new();
    while !raw_slice.is_empty() {
        let meta: Meta<'_> = match postcard::take_from_bytes::<Meta<'_>>(raw_slice) {
            Ok((meta, unused)) => {
                raw_slice = unused;
                meta
            }
            Err(err) => {
                let msg = alloc::format!("parse meta: {err}");
                log_error(&msg);
                continue;
            }
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
    bubble_sort(&mut apps);
    apps
}

/// Load metas from cache, creating it if doesn't exist.
pub fn read_metas() -> Vec<u8> {
    let name = "metas";
    let size = get_file_size(name);
    if size == 0 {
        log_debug("apps are not cached, traversing...");
        return refresh_metas();
    }
    let mut buf = vec![0; size];
    load_file(name, &mut buf);
    buf
}

/// Go through all ROMs, read their metadata, and save it in cache.
pub fn refresh_metas() -> Vec<u8> {
    let mut serialized = Vec::<u8>::new();
    let author_dirs = sudo::DirBuf::list_dirs("roms");
    for author_dir in author_dirs.iter() {
        let author_path = format!("roms/{author_dir}");
        let app_dirs = sudo::DirBuf::list_dirs(&author_path);
        for app_dir in app_dirs.iter() {
            let meta_path = format!("{author_path}/{app_dir}/_meta");
            let meta_size = sudo::get_file_size(&meta_path);
            let old_size = serialized.len();
            serialized.resize(old_size + meta_size, 0);
            sudo::load_file(&meta_path, &mut serialized[old_size..]);
        }
    }
    dump_file("metas", &serialized);
    serialized
}
