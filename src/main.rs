// # Get CLI arguments, and build a context object

mod cli;
mod templates;

use crate::cli::{Args, CliError, parse_cli_args};
use clap::Parser;
use std::process::Command;
use templates::{
    DEFAULT_TEMPLATE, TEMPLATE_DIRECTORY, Template, TemplateSource, TemplatingError, get_template,
};

#[derive(Debug)]
pub struct Options {
    output: String,
    template: TemplateSource,
    author: Option<String>,
    orcid: Option<String>,
    lang: String,
    default_template: TemplateSource,
    debug: bool,
}

pub struct AutoAuthorFromGit;

fn print_error(message: &str) -> () {
    println!("Error: {}", message);
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

    // Get the options from CLI argument, with error handling
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
                TemplatingError::NoTemplateDirectory(dir_path) => {
                    print_error(
                        format!(
                            "The {} directory does not exist.",
                            dir_path.to_str().unwrap()
                        )
                        .as_str(),
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

    // Print the options struct if in debug mode
    if options.debug {
        println!("Options struct: {:?}", options);
    }

    // Get the template, and handle any errors that may occur
    let template = match get_template(options.template, options.default_template) {
        Ok(template) => template,
        Err(error) => match error {
            TemplatingError::CouldNotFindHomeDir => {
                print_error("Could not find the home directory");
                return;
            }
            TemplatingError::CouldNotReadTemplateFile(filepath) => {
                print_error(
                    format!("Could not read file at {}", filepath.to_str().unwrap()).as_str(),
                );
                return;
            }
            TemplatingError::NoTemplateDirectory(dir_path) => {
                print_error(
                    format!(
                        "The {} directory does not exist",
                        dir_path.to_str().unwrap()
                    )
                    .as_str(),
                );
                return;
            }
            TemplatingError::TemplateNotFound(name) => {
                print_error(format!("Template with name {} not found", name).as_str());
                return;
            }
        },
    };

    if options.debug {
        println!("{:?}", template);
    }

    println!("Hello, world!");
}
