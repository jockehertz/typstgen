// This module handles templating

use crate::Options;
use crate::defaults::{
    ARTICLE_TEMPLATE_STRING, AUTHOR_PLACEHOLDER, ORCID_IMAGE, REPORT_TEMPLATE_STRING,
    TEMPLATE_DIRECTORY,
};
use dirs;
use std::fs;
use std::path::PathBuf;
use whoami::realname;

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
    CouldNotFindCfgDir,
    CouldNotReadTemplateFile(PathBuf),
}

const BUILTIN_REPORT_ARG: &str = "report";

const BUILTIN_ARTICLE_ARG: &str = "article";

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

// Reformat author name to last name, first name
fn reformat_author_name(author: String) -> String {
    let author_reformat_vec = author.split(" ").collect::<Vec<&str>>();
    let author_last_name = vec![author_reformat_vec[author_reformat_vec.len() - 1]];
    let author_other_names = author_reformat_vec[0..(author_reformat_vec.len() - 1)].join(" ");
    format!("{}, {}", author_last_name.join(""), author_other_names)
}

// Substitute template with options
fn substitute_template(template: String, options: &Options) -> String {
    let mut template = template;

    // Get the author name, infer it if git inference is enabled
    let author = match options.author.clone() {
        Some(author) => author,
        None => {
            if options.name_inference {
                match realname() {
                    Ok(username) => username,
                    Err(_) => AUTHOR_PLACEHOLDER.to_string(),
                }
            } else {
                AUTHOR_PLACEHOLDER.to_string()
            }
        }
    };

    // Substitute author name, reformatted to last name, first name
    let author_reformatted = reformat_author_name(author);
    template = template.replace("{{AUTHOR_NAME}}", &author_reformatted);

    // Substitute author ORCID ID if it exists
    // The ORCID is only declared if an ORCID ID is provided
    match options.orcid.clone() {
        Some(id) => {
            template = template.replace(
                "{{ORCID_ID}}",
                &format!(" #orcid_svg https://orcid.org/{}", id),
            );
            template = template.replace(
                "{{ORCID_ICON_DECLARATION}}",
                format!(
                    "#let orcid_svg = image(bytes(\"{}\"), width: 18pt, height: 18pt)",
                    ORCID_IMAGE
                )
                .as_str(),
            );
        }
        None => {
            template = template.replace("{{ORCID_ID}}", "");
            template = template.replace("{{ORCID_ICON_DECLARATION}}", "");
        }
    }

    template = template.replace("{{LANG}}", &options.lang);

    template
}

pub fn assemble_template(options: &Options) -> Result<String, TemplatingError> {
    let template = get_template(options.template.clone(), options.default_template.clone())?;
    let template_string = match template {
        Template::Article(content) => content,
        Template::Report(content) => content,
        Template::Custom(content) => content,
    };
    let final_string = substitute_template(template_string, &options);
    Ok(final_string)
}

// Get the source for the applied template
pub fn get_template_source(template_name: String) -> Result<TemplateSource, TemplatingError> {
    match template_name.as_str() {
        BUILTIN_REPORT_ARG => Ok(TemplateSource::BuiltinReport),
        BUILTIN_ARTICLE_ARG => Ok(TemplateSource::BuiltinArticle),
        other_name => {
            let mut template_path = PathBuf::new();
            let config_path = match dirs::config_dir() {
                Some(path) => path,
                None => return Err(TemplatingError::CouldNotFindCfgDir),
            };
            template_path.push(config_path);
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
