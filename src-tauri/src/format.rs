use crate::git;
use chrono::{DateTime, Datelike, FixedOffset, NaiveTime, TimeDelta, Utc};

#[derive(Default)]
struct FormatAcc {
    commits: Vec<FormattedCommit>,
}

pub fn format_commits(commits: Vec<git::Commit>) -> Vec<FormattedCommit> {
    commits
        .iter()
        .fold(FormatAcc::default(), |mut acc, commit| {
            acc.commits.push(FormattedCommit::from_commit(commit));
            acc
        })
        .commits
}

#[derive(Clone, Debug, serde::Serialize, specta::Type)]
#[specta(rename = "Commit")]
pub struct FormattedCommit {
    id: String,
    message: String,
    author: git::Person,
    time: FormattedTime,
    parents: Vec<String>,
    column: i32,
}

impl FormattedCommit {
    fn from_commit(commit: &git::Commit) -> Self {
        Self {
            id: commit.id.clone(),
            message: commit.message.clone(),
            author: commit.author.clone(),
            time: commit.time.into(),
            parents: commit.parents.clone(),
            column: 0,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, specta::Type)]
struct FormattedTime(String);

impl From<DateTime<FixedOffset>> for FormattedTime {
    fn from(value: DateTime<FixedOffset>) -> Self {
        let now = Utc::now();
        let midnight_today = now
            .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .unwrap();
        let start_of_week = midnight_today - TimeDelta::days(6);
        let start_of_year = midnight_today.with_day(1).unwrap().with_month(1).unwrap();

        if value >= midnight_today {
            FormattedTime(value.format("%H:%M").to_string())
        } else if value >= start_of_week {
            FormattedTime(value.format("%a, %H:%M").to_string())
        } else if value >= start_of_year {
            FormattedTime(value.format("%a, %d %b, %H:%M").to_string())
        } else {
            FormattedTime(value.format("%a, %d %b %Y, %H:%M").to_string())
        }
    }
}
