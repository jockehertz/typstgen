// # Get CLI arguments, and build a context object

mod cli;
mod config;
mod defaults;
mod templates;

use crate::cli::{Args, CliError, parse_cli_args};
use clap::Parser;
use defaults::{DEFAULT_TEMPLATE, INFERRED_NAME_REFORMAT_DEFAULT, NAME_INFERENCE_DEFAULT};
use serde::Deserialize;
use std::fs;
use templates::{TemplateSource, TemplatingError, assemble_template};

// A struct representing the options for the typstgen program
#[derive(Debug)]
pub struct Options {
    output: String,
    template: TemplateSource,
    author: Option<String>,
    orcid: Option<String>,
    lang: String,
    default_template: TemplateSource,
    debug: bool,
    name_inference: bool,
    inferred_name_reformat: bool,
}

fn print_error(message: &str) -> () {
    println!("Error: {}", message);
}

fn main() {
    // Collect the CLI arguments
    let args = Args::parse();

    // Get the options from CLI argument, with error handling
    let flag_options = match parse_cli_args(args) {
        Ok(opts) => opts,
        Err(cli_error) => match cli_error {
            CliError::TemplateError(template_error) => match template_error {
                TemplatingError::CouldNotFindCfgDir => {
                    print_error("Could not find the user's configuration directory");
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

    // Initialise the program options struct. This will have lots of match arms later on
    // due to the config file. I'm thinking main will deserialise the config, and then give a helper function in another module the
    // flag_options struct and the deserialised config to work with.
    let options = Options {
        output: flag_options.output,
        template: flag_options.template,
        author: flag_options.author,
        orcid: flag_options.orcid,
        lang: flag_options.lang,
        debug: flag_options.debug,
        default_template: DEFAULT_TEMPLATE,
        name_inference: NAME_INFERENCE_DEFAULT,
        inferred_name_reformat: INFERRED_NAME_REFORMAT_DEFAULT,
    };

    // Print the options struct if in debug mode
    if options.debug {
        println!("Options struct: {:?}", options);
    }

    let template = match assemble_template(&options) {
        Ok(template) => template,
        Err(error) => match error {
            TemplatingError::CouldNotFindCfgDir => {
                print_error("Could not find the configuration directory");
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
        println!("\n\nTEMPLATE: \n{:?}", template);
    }

    if options.debug {
        println!("\n\nWriting file...");
    }

    let file_name = match options.output.clone().ends_with(".typ") {
        true => options.output.clone(),
        false => format!("{}.typ", options.output.clone()),
    };

    let _ = fs::write(file_name, template);
}
