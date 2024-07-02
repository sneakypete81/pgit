use crate::git::{Commit, Git};
use std::{path::Path, sync::RwLock, time::Duration};
use tauri::{Invoke, Result, State, Window};

pub fn command_handler() -> impl Fn(Invoke) {
    tauri::generate_handler![init]
}

#[tauri::command]
fn init(ipc: State<Ipc>, window: Window) -> Result<()> {
    window.show()?;
    ipc.start(window);
    Ok(())
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    branches: Vec<String>,
    commits: Vec<Commit>,
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
                Self::send(
                    &window,
                    Payload {
                        branches: git.branches(),
                        commits: git.commits(),
                    },
                );
                std::thread::sleep(Duration::from_secs(1));
            }
        });
    }

    fn send(window: &Window, payload: Payload) {
        window.emit("status", payload).unwrap();
    }
}
