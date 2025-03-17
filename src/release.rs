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
    fn from_tags(tags: &[GitTag]) -> Self {
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
        let collection: Vec<ReleaseEntry> = tags
            .chunk_by(|a, b| a.target_commit == b.target_commit)
            .map(|g| ReleaseEntry::from_tags(g))
            .collect();

        ReleaseCollection(collection)
    }

    /// Populates each entry's `commits` field with the relevant commits.
    /// Entries and commits must be sorted newest to oldest.
    /// Entries without commits are filtered out.
    pub fn populate_commits(&self, commits: &[GitCommit]) -> Self {
        let mut new_self = self.clone();

        if self.0.is_empty() || commits.is_empty() {
            return new_self;
        }

        // Skip any untagged commits
        let skip_i = commits
            .iter()
            .position(|c| c.hash == self.0[0].target_commit)
            .unwrap_or(0);

        let mut entry_i = 0;

        for c in commits.iter().skip(skip_i) {
            let next_entry = self.0.get(entry_i + 1);

            if next_entry.is_some_and(|e| c.hash == e.target_commit) {
                entry_i += 1;
            }

            new_self.0[entry_i].commits.push(c.clone());
        }

        new_self.0.retain(|r| !r.commits.is_empty());

        new_self
    }
}
