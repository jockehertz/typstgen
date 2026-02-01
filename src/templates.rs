// This module handles templating

use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Template {
    DefaultTemplate,
    Report(String),
    Article(String),
    Custom(String),
}

#[derive(Debug)]
pub enum TemplateSource {
    BuiltinReport,
    BuiltinArticle,
    Custom(PathBuf),
    DefaultTemplate,
}

pub enum TemplatingError {
    TemplateNotFound(String),
    NoTemplateDirectory,
    CouldNotFindHomeDir,
    CouldNotReadTemplateFile(PathBuf),
}

const REPORT_TEMPLATE_STRING: &str = r#"
look at me, I'm a report template!
"#;

const ARTICLE_TEMPLATE_STRING: &str = r#"
look at me, I'm an article template!
"#;

pub const TEMPLATE_DIRECTORY: &str = ".typstgen/templates";

const BUILTIN_REPORT_ARG: &str = "report";

const BUILTIN_ARTICLE_ARG: &str = "article";

// Get a template, builtin or custom
pub fn get_template(
    template_source: TemplateSource,
    default_template_path: PathBuf,
) -> Result<Template, TemplatingError> {
    match template_source {
        // Builtin report template
        TemplateSource::BuiltinReport => Ok(Template::Report(String::from(REPORT_TEMPLATE_STRING))),
        // Builtin article template
        TemplateSource::BuiltinArticle => {
            Ok(Template::Article(String::from(ARTICLE_TEMPLATE_STRING)))
        }

        // Other custom template
        TemplateSource::Custom(path) => match fs::read_to_string(&path) {
            Ok(content) => Ok(Template::Custom(content)),
            Err(_) => Err(TemplatingError::CouldNotReadTemplateFile(path)),
        },

        // Default template
        TemplateSource::DefaultTemplate => match fs::read_to_string(&default_template_path) {
            Ok(content) => Ok(Template::Custom(content)),
            Err(_) => Err(TemplatingError::CouldNotReadTemplateFile(
                default_template_path,
            )),
        },
    }
}

// Get the source for the applied template
pub fn get_template_source(template_name: String) -> Result<TemplateSource, TemplatingError> {
    match template_name.as_str() {
        BUILTIN_REPORT_ARG => Ok(TemplateSource::BuiltinReport),
        BUILTIN_ARTICLE_ARG => Ok(TemplateSource::BuiltinArticle),
        other_name => {
            let mut template_path = PathBuf::new();
            let home_path = match env::var("HOME") {
                Ok(path) => path,
                Err(_) => return Err(TemplatingError::CouldNotFindHomeDir),
            };
            template_path.push(home_path);
            template_path.push(TEMPLATE_DIRECTORY);

            // If there is no template directory, return an error here
            if !template_path.exists() {
                return Err(TemplatingError::NoTemplateDirectory);
            }

            // Check if the user gave the template file with or without the .typ extenstion
            let match_regex = Regex::new("*.typ").unwrap();

            if match_regex.is_match(other_name) {
                template_path.push(other_name);
            } else {
                template_path.push(other_name);
                template_path.add_extension(".typ");
            };

            // Check that the template exists
            if template_path.exists() {
                return Ok(TemplateSource::Custom(template_path));
            } else {
                return Err(TemplatingError::TemplateNotFound(String::from(other_name)));
            }
        }
    }
}
