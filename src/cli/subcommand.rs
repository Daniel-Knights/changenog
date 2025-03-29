use super::options::ChangenogOptions;

//// Structs

pub struct Subcommand;

//// Implementations

impl Subcommand {
    /// Prints help text
    pub fn help() {
        let (longest_name_len, longest_kind_len) =
            ChangenogOptions::DEFINITIONS.iter().fold((0, 0), |acc, d| {
                (acc.0.max(d.name.len()), acc.1.max(d.kind.to_string().len()))
            });

        println!("Changenog options:");

        ChangenogOptions::DEFINITIONS.iter().for_each(|d| {
            let mut description = d.description.to_string();

            if let Some(values) = d.values {
                description = format!("{description}.  one of ['{}']", values.join("', '"));
            }

            if let Some(default) = d.default {
                description = format!("{description}.  default: '{default}'");
            };

            println!(
                "  {}{} | {}{} | {}",
                d.name,
                " ".repeat(longest_name_len - d.name.len()),
                d.kind,
                " ".repeat(longest_kind_len - d.kind.to_string().len()),
                description
            );
        });
    }

    /// Prints current version
    pub fn version() {
        println!("{}", env!("CARGO_PKG_VERSION"));
    }
}
