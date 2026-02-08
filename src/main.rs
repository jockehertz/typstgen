// Get CLI arguments, and build a context object
mod cli;
mod config;
mod defaults;
mod templates;
use std::path::PathBuf;

use crate::cli::{Args, CliError, parse_cli_args};
use clap::Parser;
use color_print::cprintln;
use config::{apply_config, apply_default_config, load_config};
use std::fs;
use templates::{TemplateSource, TemplatingError, assemble_template};

// A struct representing the options for the typstgen program
#[derive(Debug)]
pub struct Options {
    output: String,
    template: TemplateSource,
    author: String,
    orcid: String,
    lang: String,
    debug: bool,
    lib_file: PathBuf,
}

fn print_error(message: &str) -> () {
    cprintln!("<red>Error:</red> {}", message);
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
                        format!("Could not find template file at {}", filepath.display()).as_str(),
                    );
                    return;
                }
                TemplatingError::NoTemplateDirectory(dir_path) => {
                    print_error(
                        format!("The {} directory does not exist.", dir_path.display()).as_str(),
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
    // flag_options struct and the deserialised _toconfig to work with.
    let config_path: PathBuf = match dirs::config_dir() {
        Some(dir) => dir.join("typstgen/config.toml"),
        None => {
            print_error("Could not find the configuration directory");
            return;
        }
    };

    let app_config = load_config(&config_path);

    // Load the config file if it exists, otherwise load the default configuration
    let options = match app_config {
        Some(config) => {
            if flag_options.debug {
                cprintln!("<green>Config file loaded</green>");
            }
            apply_config(&config, flag_options)
        }
        None => {
            if flag_options.debug {
                cprintln!("<yellow>No config file found, loading default configuration</yellow>");
            }
            apply_default_config(flag_options)
        }
    };

    // Print the options struct if in debug mode
    if options.debug {
        println!("Options struct: {:?}", options);
    }

    // Assemble the template given the options
    let template = match assemble_template(&options) {
        Ok(template) => template,
        Err(error) => match error {
            TemplatingError::CouldNotFindCfgDir => {
                print_error("Could not find the configuration directory");
                return;
            }
            TemplatingError::CouldNotReadTemplateFile(filepath) => {
                print_error(format!("Could not read file at {}", filepath.display()).as_str());
                return;
            }
            TemplatingError::NoTemplateDirectory(dir_path) => {
                print_error(
                    format!("The {} directory does not exist", dir_path.display()).as_str(),
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

    let file_name = match options.output.ends_with(".typ") {
        true => options.output.clone(),
        false => format!("{}.typ", options.output),
    };

    let lib_file_path = match dirs::config_dir() {
        Some(path) => match path.join("typstgen").join(&options.lib_file).exists() {
            true => Some(path.join("typstgen").join(&options.lib_file)),
            false => None,
        },
        None => None,
    };
    // Copy the library file if it exists
    match lib_file_path {
        Some(path) => {
            let copy_lib = fs::copy(path, options.lib_file.clone());
            match copy_lib {
                Ok(_) => cprintln!("<green>Library file copied successfully</green>"),
                Err(_) => print_error("Could not copy library file"),
            }
        }
        None => {
            if options.debug {
                print_error("Could not find library file");
            }
        }
    }

    // Write the template to the file
    let written = fs::write(file_name, template);

    if options.debug {
        match written {
            Ok(_) => cprintln!("<green>File written successfully</green>"),
            Err(_) => print_error("Could not write file"),
        }
    }
}
