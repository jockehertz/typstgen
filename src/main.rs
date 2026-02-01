// # Get CLI arguments, and build a context object

mod cli;
mod templates;

use crate::cli::{Args, CliError, parse_cli_args};
use clap::Parser;
use std::process::Command;
use templates::{TEMPLATE_DIRECTORY, TemplateSource, TemplatingError};

#[derive(Debug)]
pub struct Options {
    author: Option<String>,
    template: TemplateSource,
    orcid: Option<String>,
    debug: bool,
}

pub struct AutoAuthorFromGit;

fn print_error(message: &str) -> () {
    println!("{}", message);
}

// Gets the git username
fn get_git_username() -> Option<String> {
    let output = Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok();

    match output {
        Some(output) => String::from_utf8(output.stdout).ok(),
        None => None,
    }
}

fn main() {
    // Collect the CLI arguments
    let args = Args::parse();

    let options = match parse_cli_args(args) {
        Ok(opts) => opts,
        Err(cli_error) => match cli_error {
            CliError::TemplateError(template_error) => match template_error {
                TemplatingError::CouldNotFindHomeDir => {
                    print_error("Could not find the user's home directory");
                    return;
                }
                TemplatingError::CouldNotReadTemplateFile(filepath) => {
                    print_error(
                        format!(
                            "Could not find template file at {}",
                            filepath.to_str().unwrap()
                        )
                        .as_str(),
                    );
                    return;
                }
                TemplatingError::NoTemplateDirectory => {
                    print_error(
                        format!("The {} directory does not exist.", TEMPLATE_DIRECTORY).as_str(),
                    );
                    return;
                }
                TemplatingError::TemplateNotFound(filepath) => {
                    print_error(format!("No template was found at {}", filepath.as_str()).as_str());
                    return;
                }
            },
        },
    };

    if options.debug {
        println!("{:?}", options);
    }

    println!("Hello, world!");
}
