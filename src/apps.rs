use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use firefly_meta::Meta;
use firefly_rust::*;

pub struct App {
    pub id: String,
    pub author_id: String,
    pub name: String,
    pub author_name: String,
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
            if meta.author_id == "sys" && meta.app_id == "launcher" {
                continue;
            }
            apps.push(App {
                id: meta.app_id.to_string(),
                author_id: meta.author_id.to_string(),
                name: meta.app_name.to_string(),
                author_name: meta.author_name.to_string(),
            });
        }
    }
    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps
}
