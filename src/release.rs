use crate::{
    cli::options::ChangenogOptions,
    git::{commit::GitCommit, tag::GitTag},
};

//// Structs

#[derive(Debug, Clone)]
pub struct ReleaseEntry {
    pub tags: Vec<GitTag>,
    pub commits: Vec<GitCommit>,
}

#[derive(Debug, Clone)]
pub struct ReleaseCollection(pub Vec<ReleaseEntry>);

//// Implementations

impl ReleaseEntry {
    fn new() -> ReleaseEntry {
        ReleaseEntry {
            tags: vec![],
            commits: vec![],
        }
    }
}

impl ReleaseCollection {
    /// Loops over all tags, gets the relevant commits for each, and builds entries.
    /// Applies all tag and commit filters to each entry.
    /// If tags or commits are empty after filtering, the entry itself is also excluded.
    /// Tags must be sorted newest to oldest.
    pub fn from(
        tags: &[GitTag],
        prev_entry_tag: &Option<String>,
        opts: &ChangenogOptions,
    ) -> ReleaseCollection {
        let mut collection = ReleaseCollection(vec![]);
        let mut entry = ReleaseEntry::new();

        for i in 0..tags.len() {
            // Limit by --max-entries
            if collection.0.len() >= opts.max_entries {
                break;
            }

            let curr_tag = &tags[i];

            let prev_tag_name = tags
                .get(i + 1)
                .map(|t| t.name.clone())
                .or(prev_entry_tag.clone());

            let commits = GitCommit::get_all_for_tag(curr_tag, &prev_tag_name, opts);
            let filtered_commits = GitCommit::apply_filters(&commits, &opts.commit_filters);

            // Apply tag filters
            if curr_tag.passes_filters(&opts.tag_filters) {
                entry.tags.push(curr_tag.clone());
            }

            if filtered_commits.is_empty() {
                // Next tag will be added along with its commits
                continue;
            }

            if entry.tags.is_empty() {
                // Merge commits with previous entry (or empty current entry if this is the first one)
                let prev_entry = collection.0.last_mut().unwrap_or(&mut entry);

                prev_entry.commits.extend(filtered_commits);
            } else {
                // Tag ordering can be unstable across machines, so sort by name
                entry.tags.sort_by(|a, b| a.name.cmp(&b.name));
                entry.commits.extend(filtered_commits);

                collection.0.push(entry);

                entry = ReleaseEntry::new();
            }
        }

        collection
    }
}
