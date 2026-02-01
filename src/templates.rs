// This module handles templating

use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Template {
    Report(String),
    Article(String),
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum TemplateSource {
    BuiltinReport,
    BuiltinArticle,
    Custom(PathBuf),
    DefaultTemplate,
}

pub enum TemplatingError {
    TemplateNotFound(String),
    NoTemplateDirectory(PathBuf),
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

pub const DEFAULT_TEMPLATE: TemplateSource = TemplateSource::BuiltinReport;

// Get a template, builtin or custom
pub fn get_template(
    template_source: TemplateSource,
    default_template: TemplateSource,
) -> Result<Template, TemplatingError> {
    let actual_source = match template_source {
        TemplateSource::DefaultTemplate => default_template,
        other => other,
    };

    match actual_source {
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
        TemplateSource::DefaultTemplate => unreachable!(),
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
                return Err(TemplatingError::NoTemplateDirectory(template_path));
            }

            // Check if the user gave the template file with or without the .typ extenstion

            if other_name.ends_with(".typ") {
                template_path.push(other_name);
            } else {
                template_path.push(other_name);
                template_path.set_extension("typ");
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
