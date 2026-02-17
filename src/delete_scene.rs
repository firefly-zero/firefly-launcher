use crate::*;
use firefly_rust::sudo;

pub fn init(state: &State) {
    let app = &state.apps[state.pos];
    let full_id = alloc::format!("{}.{}", app.author_id, app.id);
    let full_id = full_id.as_bytes();
    sudo::dump_file("data/sys/remover/etc/target", full_id);
    sudo::run_app("sys", "remover");
}

pub fn update(_state: &State) {}

pub fn render(_state: &State) {}
