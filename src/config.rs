use crate::Options;
use crate::cli::FlagOptions;
use crate::defaults::{
    DEFAULT_LIB_FILE, DEFAULT_ORCID, DEFAULT_OUTPUT, DEFAULT_TEMPLATE,
    INFERRED_NAME_REFORMAT_DEFAULT, NAME_INFERENCE_DEFAULT,
};
use crate::templates::TemplateSource;
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    default_output: Option<String>,
    default_template: Option<String>,
    inferred_name_reformat: Option<bool>,
    name_inference: Option<bool>,
    orcid: Option<String>,
    email: Option<String>,
    default_author: Option<String>,
    lib_file: Option<String>,
}

pub fn load_config(config_path: &PathBuf) -> Option<Config> {
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
        inferred_name_reformat: config.inferred_name_reformat,
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
        output: match input_options.output {
            Some(output) => output,
            None => match config.default_output.clone() {
                Some(output) => output,
                None => String::from(DEFAULT_OUTPUT),
            },
        },

        // Get the template from the config if none is provided
        template: match input_options.template {
            Some(template) => template,
            None => match &config.default_template {
                Some(name) => {
                    let default_template_path =
                        dirs::config_dir().unwrap().join("typstgen/templates");
                    let template_path_reformatted = match name.ends_with("typ") {
                        true => default_template_path.join(name),
                        false => default_template_path.join(format!("{}.typ", name)),
                    };
                    TemplateSource::Custom(template_path_reformatted)
                }
                None => DEFAULT_TEMPLATE,
            },
        },

        // Get whether the name is to be inferred
        name_inference: match config.name_inference.clone() {
            Some(inference) => inference,
            None => NAME_INFERENCE_DEFAULT,
        },

        // Get whether the name is to be reformatted
        inferred_name_reformat: match config.inferred_name_reformat.clone() {
            Some(reformat) => reformat,
            None => INFERRED_NAME_REFORMAT_DEFAULT,
        },

        // Get the orcid from the directory if not provided
        orcid: match input_options.orcid {
            Some(orcid) => orcid,
            None => match config.orcid.clone() {
                Some(orcid) => orcid,
                None => String::from(DEFAULT_ORCID),
            },
        },

        // Get the author from the config if not provided
        author: match input_options.author {
            Some(author) => Some(author),
            None => match config.default_author.clone() {
                Some(author) => Some(author),
                None => None,
            },
        },

        // Get the language
        lang: input_options.lang,

        // Get the debug flag
        debug: input_options.debug,

        lib_file: match config.lib_file.clone() {
            Some(lib_file) => lib_file,
            None => String::from(DEFAULT_LIB_FILE),
        },
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
        author: input_options.author,
        orcid: match input_options.orcid {
            Some(orcid) => orcid,
            None => String::from(DEFAULT_ORCID),
        },
        name_inference: NAME_INFERENCE_DEFAULT,
        inferred_name_reformat: INFERRED_NAME_REFORMAT_DEFAULT,
        lib_file: String::from(DEFAULT_LIB_FILE),
    }
}
