use std::collections::HashMap;

use crate::git::{commit::GitCommit, tag::GitTag};

//// Structs

#[derive(Debug, Clone)]
pub struct ReleaseEntry {
    pub tags: Vec<GitTag>,
    pub date: String,
    pub target_commit: String,
    pub commits: Vec<GitCommit>,
}

#[derive(Debug, Clone)]
pub struct ReleaseCollection(pub Vec<ReleaseEntry>);

//// Implementations

impl ReleaseEntry {
    pub fn from_tags(tags: &[GitTag]) -> Self {
        Self {
            tags: tags.to_vec(),
            date: tags[0].date.clone(),
            target_commit: tags[0].target_commit.clone(),
            commits: vec![],
        }
    }
}

impl ReleaseCollection {
    pub fn from_tags(tags: &[GitTag]) -> Self {
        let mut target_commit_map: HashMap<String, Vec<GitTag>> = HashMap::new();

        tags.iter().for_each(|t| {
            let target_commit_val = target_commit_map
                .entry(t.target_commit.clone())
                .or_insert_with(Vec::new);

            target_commit_val.push(t.clone());
        });

        let releases = target_commit_map
            .iter()
            .map(|(_, v)| ReleaseEntry::from_tags(v))
            .collect();

        ReleaseCollection(releases)
    }

    /// Populates each entry's `commit` field with the relevant commits.
    /// `entries` and `commits` must be sorted newest to oldest.
    /// Entries without commits are filtered out.
    pub fn populate_commits(&self, commits: &[GitCommit]) -> Self {
        if self.0.is_empty() || commits.is_empty() {
            return self.clone();
        }

        // Skip any untagged commits
        let skip_i = commits
            .iter()
            .position(|c| c.hash == self.0[0].target_commit)
            .unwrap_or(0);

        let mut new_self = self.clone();
        let mut entry_i = 0;

        for c in commits.iter().skip(skip_i) {
            let next_entry = self.0.get(entry_i + 1);

            if c.hash == new_self.0[entry_i].target_commit {
                new_self.0[entry_i].commits.push(c.clone());
            } else if next_entry.is_some() && c.hash == next_entry.unwrap().target_commit {
                entry_i += 1;
            } else {
                new_self.0[entry_i].commits.push(c.clone());
            }
        }

        new_self.0.retain(|r| !r.commits.is_empty());

        new_self
    }
}
