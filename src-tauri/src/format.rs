use crate::git;

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
    time: String,
    parents: Vec<String>,
    column: i32,
}

impl FormattedCommit {
    fn from_commit(commit: &git::Commit) -> Self {
        Self {
            id: commit.id.clone(),
            message: commit.message.clone(),
            author: commit.author.clone(),
            time: commit.time.to_rfc3339(),
            parents: commit.parents.clone(),
            column: 0,
        }
    }
}
