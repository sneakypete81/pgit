use git2::{Repository, Sort};
use std::path::Path;

pub struct Git {
    repo: Repository,
}

impl Git {
    pub fn open(path: &Path) -> Self {
        Self {
            repo: Repository::open(path).unwrap(),
        }
    }

    pub fn branches(&self) -> Vec<String> {
        self.repo
            .branches(None)
            .unwrap()
            .map(|b| b.unwrap().0.name().unwrap().unwrap().to_owned())
            .collect()
    }

    pub fn commits(&self) -> Vec<String> {
        let mut walker = self.repo.revwalk().unwrap();
        walker.push_glob("*").unwrap();
        walker.set_sorting(Sort::TOPOLOGICAL | Sort::TIME).unwrap();
        let oids = walker.map(|r| r.unwrap());
        oids.map(|oid| {
            self.repo
                .find_commit(oid)
                .unwrap()
                .message()
                .unwrap()
                .to_owned()
        })
        .collect()
    }
}
