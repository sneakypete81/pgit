use chrono::{DateTime, FixedOffset};
use git2::{Repository, Sort};
use std::path::Path;

#[derive(Clone, serde::Serialize)]
pub struct Commit {
    message: String,
    author: Person,
    time: Time,
    column: i32,
}

#[derive(Clone, serde::Serialize)]
struct Person {
    name: Option<String>,
    email: Option<String>,
}

impl From<git2::Signature<'_>> for Person {
    fn from(signature: git2::Signature) -> Person {
        Person {
            name: signature.name().map(|n| n.to_owned()),
            email: signature.email().map(|e| e.to_owned()),
        }
    }
}

#[derive(Clone, serde::Serialize)]
struct Time(String);

impl TryFrom<git2::Time> for Time {
    type Error = &'static str;

    fn try_from(time: git2::Time) -> Result<Time, &'static str> {
        Ok(Time(
            DateTime::from_timestamp(time.seconds(), 0)
                .ok_or("inavlid timestamp")?
                .with_timezone(
                    &FixedOffset::east_opt(time.offset_minutes() * 60).ok_or("invalid timezone")?,
                )
                .to_rfc3339(),
        ))
    }
}

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

    pub fn commits(&self) -> Vec<Commit> {
        let mut walker = self.repo.revwalk().unwrap();
        walker.push_glob("*").unwrap();
        walker.set_sorting(Sort::TOPOLOGICAL | Sort::TIME).unwrap();
        let oids = walker.map(|r| r.unwrap());
        let commits = oids.map(|oid| self.repo.find_commit(oid).unwrap());
        commits
            .map(|c| Commit {
                message: c.message().unwrap().to_owned(),
                author: c.author().into(),
                time: c.time().try_into().unwrap(),
                column: 0,
            })
            .collect()
    }
}
