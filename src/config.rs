use crate::Options;
use crate::cli::FlagOptions;
use crate::defaults::{
    AUTHOR_PLACEHOLDER, DEFAULT_EMAIL, DEFAULT_LIB_FILE, DEFAULT_ORCID, DEFAULT_OUTPUT,
    DEFAULT_TEMPLATE, NAME_INFERENCE_DEFAULT,
};
use crate::templates::TemplateSource;
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use whoami::realname;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    default_output: Option<String>,
    default_template: Option<String>,
    name_inference: Option<bool>,
    orcid: Option<String>,
    email: Option<String>,
    default_author: Option<String>,
    lib_file: Option<String>,
}

// Process author name, infer it if git inference is enabled
fn process_author_name(
    author: Option<String>,
    config_default: Option<String>,
    infer_name: Option<bool>,
) -> String {
    author.or(config_default).unwrap_or_else(|| {
        if infer_name.unwrap_or(NAME_INFERENCE_DEFAULT) {
            realname().unwrap_or(String::from(AUTHOR_PLACEHOLDER))
        } else {
            String::from(AUTHOR_PLACEHOLDER)
        }
    })
}

pub fn load_config(config_path: impl AsRef<Path>) -> Option<Config> {
    // Implementation goes here
    let config_file = match fs::read_to_string(config_path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    let config: Config = match toml::from_str(&config_file) {
        Ok(config) => config,
        Err(_) => return None,
    };
    Some(Config {
        default_output: config.default_output,
        default_template: config.default_template,
        name_inference: config.name_inference,
        orcid: config.orcid,
        email: config.email,
        default_author: config.default_author,
        lib_file: config.lib_file,
    })
}

pub fn apply_config(config: &Config, input_options: FlagOptions) -> Options {
    Options {
        // Get the output name from the config if none is provided
        output: input_options
            .output
            .or(config.default_output.clone())
            .unwrap_or(String::from(DEFAULT_OUTPUT)),

        // Get the template from the config if none is provided
        template: match input_options.template {
            Some(template) => template,
            None => match &config.default_template {
                Some(name) => {
                    let default_template_path = match dirs::config_dir() {
                        Some(dir) => Some(dir.join("typstgen/templates")),
                        None => None,
                    };
                    match default_template_path {
                        Some(default_template_path) => {
                            let template_path_reformatted = match name.ends_with("typ") {
                                true => default_template_path.join(name),
                                false => default_template_path.join(format!("{}.typ", name)),
                            };
                            TemplateSource::Custom(template_path_reformatted)
                        }
                        None => DEFAULT_TEMPLATE,
                    }
                }
                None => DEFAULT_TEMPLATE,
            },
        },

        // Get the orcid from the directory if not provided
        orcid: input_options
            .orcid
            .or(config.orcid.clone())
            .unwrap_or(String::from(DEFAULT_ORCID)),

        // Get the author from the config if not provided
        author: process_author_name(
            input_options.author,
            config.default_author.clone(),
            config.name_inference.clone(),
        ),

        // Get the language
        lang: input_options.lang,

        // Get the debug flag
        debug: input_options.debug,

        lib_file: match config.lib_file.clone() {
            Some(lib_file) => PathBuf::from(lib_file),
            None => PathBuf::from(DEFAULT_LIB_FILE),
        },

        email: config.email.clone().unwrap_or(String::from(DEFAULT_EMAIL)),
    }
}

pub fn apply_default_config(input_options: FlagOptions) -> Options {
    Options {
        output: match input_options.output {
            Some(output) => output,
            None => String::from(DEFAULT_OUTPUT),
        },
        lang: input_options.lang,
        debug: input_options.debug,
        template: match input_options.template {
            Some(template) => template,
            None => DEFAULT_TEMPLATE,
        },
        author: input_options
            .author
            .unwrap_or(String::from(AUTHOR_PLACEHOLDER)),
        orcid: match input_options.orcid {
            Some(orcid) => orcid,
            None => String::from(DEFAULT_ORCID),
        },
        lib_file: PathBuf::from(DEFAULT_LIB_FILE),
        email: String::from(DEFAULT_EMAIL),
    }
}
