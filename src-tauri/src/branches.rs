use std::{sync::RwLock, time::Duration};

use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    branches: Vec<String>,
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
            let mut branches = vec!["main".into(), "private/pete/feature".into()];
            loop {
                branches.push(format!("another-branch-{}", branches.len()));
                Self::emit_branches(&window, branches.clone());
                std::thread::sleep(Duration::from_secs(1));
            }
        });
    }

    fn emit_branches(window: &Window, branches: Vec<String>) {
        window.emit("branches", Payload { branches }).unwrap();
    }
}
