use crate::format::{format_commits, FormattedCommit};
use crate::git::Git;
use serde::Serialize;
use std::{path::Path, sync::RwLock, time::Duration};
use tauri::{ipc::Invoke, State, Window};
use tauri_specta::{ts, Event};

pub fn build() -> (impl Fn(Invoke) -> bool, impl FnOnce(&tauri::App)) {
    let builder = ts::builder()
        .commands(tauri_specta::collect_commands![init])
        .events(tauri_specta::collect_events![Status]);

    #[cfg(debug_assertions)]
    let builder = builder.path("../src/lib/bindings.ts");

    builder.build().unwrap()
}

#[tauri::command]
#[specta::specta]
pub fn init(ipc: State<Ipc>, window: Window) {
    window.show().unwrap();
    ipc.start(window);
}

#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
pub struct Status {
    branches: Vec<String>,
    commits: Vec<FormattedCommit>,
}

#[derive(Default)]
pub struct Ipc {
    started: RwLock<bool>,
}

impl Ipc {
    pub fn start(&self, window: Window) {
        if *self.started.read().unwrap() {
            println!("Already started");
            return;
        }
        Self::spawn(window);
        *self.started.write().unwrap() = true;
    }

    fn spawn(window: Window) {
        std::thread::spawn(move || {
            let git = Git::open(Path::new("/Users/peteburgers/projects/pgit"));
            loop {
                let status = Status {
                    branches: git.branches(),
                    commits: format_commits(git.commits()),
                };
                status.emit(&window).unwrap();
                std::thread::sleep(Duration::from_secs(1));
            }
        });
    }
}
