use crate::Options;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub enum ConfigError {
    FileNotFound,
    InvalidFormat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    default_output: Option<String>,
    default_template: Option<String>,
    inferred_name_reformat: Option<bool>,
    name_inference: Option<bool>,
    orcid: Option<String>,
    email: Option<String>,
    default_author: Option<String>,
}

pub fn read_config(config_path: &PathBuf) -> Result<Config, ConfigError> {
    // Implementation goes here
    let config_file = match fs::read_to_string(config_path) {
        Ok(file) => file,
        Err(_) => return Err(ConfigError::FileNotFound),
    };
    let config: Config = match toml::from_str(&config_file) {
        Ok(config) => config,
        Err(_) => return Err(ConfigError::InvalidFormat),
    };
    Ok(Config {
        default_output: config.default_output,
        default_template: config.default_template,
        inferred_name_reformat: config.inferred_name_reformat,
        name_inference: config.name_inference,
        orcid: config.orcid,
        email: config.email,
        default_author: config.default_author,
    })
}

pub fn apply_config_to_options(config: &Config, input_options: &Options) -> Options {
    let options = Options {
        output: match input_options.output {
            Some(output) => output,
            None => None,
        }

    }
}
