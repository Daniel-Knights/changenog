use std::{env::current_dir, fmt::Display, path::PathBuf};

use super::options::options::{ChangenogOptions, CliOption};

//// Structs

pub struct Validator;

#[derive(Debug)]
pub struct ValidateArgError(String);

impl Display for ValidateArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

//// Implementations

impl Validator {
    /// Returns result indicating whether `val` is a valid value for `arg`
    pub fn validate_arg(arg: &CliOption, val: &str) -> Result<(), ValidateArgError> {
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
            _ => Ok(()),
        }
    }

    //// Private

    fn validate_overwrite(val: &str) -> Result<(), ValidateArgError> {
        Validator::validate_boolean(ChangenogOptions::OVERWRITE, val)
    }

    fn validate_root(val: &str) -> Result<(), ValidateArgError> {
        let curr_dir = current_dir().unwrap();

        let is_within_cwd = PathBuf::from(val)
            .canonicalize()
            .unwrap()
            .starts_with(curr_dir);

        if !is_within_cwd {
            return Err(ValidateArgError(format!(
                "root must be within the current working directory: '{}={val}'",
                ChangenogOptions::ROOT.name,
            )));
        }

        Ok(())
    }

    fn validate_output(val: &str) -> Result<(), ValidateArgError> {
        Validator::validate_values(ChangenogOptions::OUTPUT, val)
    }
    fn validate_no_links(val: &str) -> Result<(), ValidateArgError> {
        Validator::validate_boolean(ChangenogOptions::NO_LINKS, val)
    }

    fn validate_max_entries(val: &str) -> Result<(), ValidateArgError> {
        if let Err(_) = val.parse::<usize>() {
            return Err(ValidateArgError(format!(
                "unable to parse max-entries: '{}={val}'",
                ChangenogOptions::MAX_ENTRIES.name,
            )));
        }

        Ok(())
    }

    fn validate_remote_url(val: &str) -> Result<(), ValidateArgError> {
        Validator::validate_not_empty(ChangenogOptions::REMOTE_URL, val)
    }

    fn validate_tag_filter_regex(val: &str) -> Result<(), ValidateArgError> {
        Validator::validate_not_empty(ChangenogOptions::TAG_FILTER_REGEX, val)
    }

    fn validate_commit_filter_regex(val: &str) -> Result<(), ValidateArgError> {
        Validator::validate_not_empty(ChangenogOptions::COMMIT_FILTER_REGEX, val)
    }

    fn validate_commit_filter_preset(val: &str) -> Result<(), ValidateArgError> {
        Validator::validate_values(ChangenogOptions::COMMIT_FILTER_PRESET, val)
    }

    //// Helpers

    /// Validates boolean args
    fn validate_boolean(arg: &CliOption, val: &str) -> Result<(), ValidateArgError> {
        if !val.is_empty() {
            return Err(ValidateArgError(format!(
                "unexpected value for boolean option: '{}={val}'",
                arg.name,
            )));
        }

        Ok(())
    }

    /// Validates value against list of accepted values
    fn validate_values(arg: &CliOption, val: &str) -> Result<(), ValidateArgError> {
        if arg.values.is_some() && !arg.values.unwrap().contains(&val) {
            return Err(ValidateArgError(format!(
                "invalid value for option: '{}={val}'.  must be one of ['{}']",
                arg.name,
                arg.values.unwrap().join("', '")
            )));
        }

        Ok(())
    }

    /// Validates that the value isn't empty
    fn validate_not_empty(arg: &CliOption, val: &str) -> Result<(), ValidateArgError> {
        if val.is_empty() {
            return Err(ValidateArgError(format!(
                "expected value for arg: '{}'",
                arg.name
            )));
        }

        Ok(())
    }
}
