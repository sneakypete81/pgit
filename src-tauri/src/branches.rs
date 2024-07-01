use crate::git::Git;
use std::{path::Path, sync::RwLock, time::Duration};
use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    branches: Vec<String>,
    commits: Vec<String>,
}

#[derive(Default)]
pub struct BranchEmitter {
    started: RwLock<bool>,
}

impl BranchEmitter {
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
                let branches = git.branches();
                let commits = git.commits();
                println!("{:?}", commits);
                Self::emit_branches(&window, Payload { branches, commits });
                std::thread::sleep(Duration::from_secs(1));
            }
        });
    }

    fn emit_branches(window: &Window, payload: Payload) {
        window.emit("branches", payload).unwrap();
    }
}
