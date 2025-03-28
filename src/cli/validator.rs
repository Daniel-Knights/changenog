use std::{env::current_dir, path::PathBuf};

use crate::log::log_exit;

use super::options::options::{ChangenogOptions, CliOption};

//// Structs

pub struct Validator;

//// Implementations

impl Validator {
    /// Returns bool indicating whether `val` is a valid value for `arg`
    pub fn validate_arg(arg: &CliOption, val: &str) -> bool {
        match arg.name {
            n if n == ChangenogOptions::OVERWRITE.name => Validator::validate_overwrite(val),
            n if n == ChangenogOptions::ROOT.name => Validator::validate_root(val),
            n if n == ChangenogOptions::OUTPUT.name => Validator::validate_output(val),
            n if n == ChangenogOptions::NO_LINKS.name => Validator::validate_no_links(val),
            n if n == ChangenogOptions::MAX_ENTRIES.name => Validator::validate_max_entries(val),
            n if n == ChangenogOptions::REMOTE_URL.name => Validator::validate_remote_url(val),
            n if n == ChangenogOptions::TAG_FILTER_REGEX.name => {
                Validator::validate_tag_filter_regex(val)
            }
            n if n == ChangenogOptions::COMMIT_FILTER_REGEX.name => {
                Validator::validate_commit_filter_regex(val)
            }
            n if n == ChangenogOptions::COMMIT_FILTER_PRESET.name => {
                Validator::validate_commit_filter_preset(val)
            }
            _ => false,
        }
    }

    //// Private

    fn validate_overwrite(val: &str) -> bool {
        Validator::validate_boolean(ChangenogOptions::OVERWRITE, val)
    }

    fn validate_root(val: &str) -> bool {
        let curr_dir = current_dir().unwrap();

        let is_within_cwd = PathBuf::from(val)
            .canonicalize()
            .unwrap()
            .starts_with(curr_dir);

        if !is_within_cwd {
            log_exit(&format!(
                "root must be within the current working directory: '{}={}'",
                ChangenogOptions::ROOT.name,
                val
            ));

            return false;
        }

        true
    }

    fn validate_output(val: &str) -> bool {
        Validator::validate_values(ChangenogOptions::OUTPUT, val)
    }
    fn validate_no_links(val: &str) -> bool {
        Validator::validate_boolean(ChangenogOptions::NO_LINKS, val)
    }

    fn validate_max_entries(val: &str) -> bool {
        if !val.parse::<usize>().is_ok() {
            log_exit(&format!(
                "invalid value: '{}={}'",
                ChangenogOptions::MAX_ENTRIES.name,
                val
            ));

            return false;
        }

        true
    }

    fn validate_remote_url(val: &str) -> bool {
        Validator::validate_not_empty(ChangenogOptions::REMOTE_URL, val)
    }

    fn validate_tag_filter_regex(val: &str) -> bool {
        Validator::validate_not_empty(ChangenogOptions::TAG_FILTER_REGEX, val)
    }

    fn validate_commit_filter_regex(val: &str) -> bool {
        Validator::validate_not_empty(ChangenogOptions::COMMIT_FILTER_REGEX, val)
    }

    fn validate_commit_filter_preset(val: &str) -> bool {
        Validator::validate_values(ChangenogOptions::COMMIT_FILTER_PRESET, val)
    }

    //// Helpers

    /// Validates boolean args
    fn validate_boolean(arg: &CliOption, val: &str) -> bool {
        if !val.is_empty() {
            log_exit(&format!(
                "unexpected value for boolean option: '{}={}'",
                arg.name, val
            ));

            return false;
        }

        true
    }

    /// Validates value against list of accepted values
    fn validate_values(arg: &CliOption, val: &str) -> bool {
        if arg.values.is_some() && !arg.values.unwrap().contains(&val) {
            log_exit(&format!(
                "invalid value for option: '{}={}'.  must be one of ['{}']",
                arg.name,
                val,
                arg.values.unwrap().join("', '")
            ));

            return false;
        }

        true
    }

    /// Validates that the value isn't empty
    fn validate_not_empty(arg: &CliOption, val: &str) -> bool {
        if val.is_empty() {
            log_exit(&format!("expected value for arg: '{}'", arg.name));

            return false;
        }

        true
    }
}
