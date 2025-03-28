use std::{fmt::Display, path::PathBuf};

use fancy_regex::Regex;

//// Structs

pub struct ChangenogOptions {
    pub overwrite: bool,
    // TODO: does root work with nested git directories and symlinks?
    pub root: PathBuf,
    pub output: String,
    pub no_links: bool,
    pub max_entries: usize,
    pub remote_url: Option<String>,
    pub tag_filters: Vec<Regex>,
    pub commit_filters: Vec<Regex>,
}

#[derive(PartialEq)]
pub enum CliOptionKind {
    Boolean,
    String,
    Number,
    Regex,
}

pub struct CliOption {
    pub name: &'static str,
    pub kind: CliOptionKind,
    pub description: &'static str,
    pub values: Option<&'static [&'static str]>,
    pub default: Option<&'static str>,
}

//// Implementations

impl ChangenogOptions {
    pub const DEFINITIONS: [CliOption; 9] = [
      CliOption {
          name: "--overwrite",
          kind: CliOptionKind::Boolean,
          description: "overwrite existing changelog",
          values: None,
          default: None,
      },
      CliOption {
          name: "--root",
          kind: CliOptionKind::String,
          description: "root dir relative to the current working directory.  default: current working directory",
          values: None,
          default: None,
      },
      CliOption {
          name: "--output",
          kind: CliOptionKind::String,
          description: "output of the generated changelog",
          values: Some(&["file", "stdout"]),
          default: Some("file"),
      },
      CliOption {
          name: "--no-links",
          kind: CliOptionKind::Boolean,
          description: "disable links",
          values: None,
          default: None,
      },
      CliOption {
          name: "--remote-url",
          kind: CliOptionKind::String,
          description: "remote URL to use for links.  default: origin",
          values: None,
          default: None,
      },
      CliOption {
          name: "--max-entries",
          kind: CliOptionKind::Number,
          description: "maximum number of entries to process",
          values: None,
          default: Some("100"),
      },
      CliOption {
          name: "--tag-filter-regex",
          kind: CliOptionKind::Regex,
          description: "regex pattern(s) that each tag must match to be included",
          values: None,
          default: None,
      },
      CliOption {
          name: "--commit-filter-regex",
          kind: CliOptionKind::Regex,
          description: "regex pattern(s) that each commit must match to be included",
          values: None,
          default: None,
      },
      CliOption {
          name: "--commit-filter-preset",
          kind: CliOptionKind::String,
          description: "filter preset(s) to use",
          values: Some(&[
              "angular",
              "angular-readme-only-docs",
              "no-changelog",
              "no-semver",
          ]),
          default: None,
      },
  ];

    pub const OVERWRITE: &'static CliOption = &Self::DEFINITIONS[0];
    pub const ROOT: &'static CliOption = &Self::DEFINITIONS[1];
    pub const OUTPUT: &'static CliOption = &Self::DEFINITIONS[2];
    pub const NO_LINKS: &'static CliOption = &Self::DEFINITIONS[3];
    pub const REMOTE_URL: &'static CliOption = &Self::DEFINITIONS[4];
    pub const MAX_ENTRIES: &'static CliOption = &Self::DEFINITIONS[5];
    pub const TAG_FILTER_REGEX: &'static CliOption = &Self::DEFINITIONS[6];
    pub const COMMIT_FILTER_REGEX: &'static CliOption = &Self::DEFINITIONS[7];
    pub const COMMIT_FILTER_PRESET: &'static CliOption = &Self::DEFINITIONS[8];
}

impl Display for CliOptionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            Self::Boolean => "boolean",
            Self::String => "string",
            Self::Number => "number",
            Self::Regex => "regex",
        };

        write!(f, "{}", kind)
    }
}
