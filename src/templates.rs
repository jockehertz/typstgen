// This module handles templating

use crate::Options;
use crate::cli::FlagOptions;
use crate::defaults::{
    ARTICLE_TEMPLATE_STRING, AUTHOR_PLACEHOLDER, ORCID_ICON_SIZE_PT, ORCID_IMAGE,
    REPORT_TEMPLATE_STRING, TEMPLATE_DIRECTORY,
};
use dirs;
use std::fs;
use std::path::PathBuf;
use whoami::realname;

// An enum representing the different types of templates available
#[derive(Debug, Clone)]
pub enum Template {
    Report(String),
    Article(String),
    Custom(String),
}

// An enum representing the different sources of templates available
#[derive(Debug, Clone)]
pub enum TemplateSource {
    BuiltinReport,
    BuiltinArticle,
    Custom(PathBuf),
}

// An enum representing the different errors that can occur during templating
pub enum TemplatingError {
    TemplateNotFound(String),
    NoTemplateDirectory(PathBuf),
    CouldNotFindCfgDir,
    CouldNotReadTemplateFile(PathBuf),
}

const BUILTIN_REPORT_ARG: &str = "report";

const BUILTIN_ARTICLE_ARG: &str = "article";

// Get a template, builtin or custom
pub fn get_template(template_source: TemplateSource) -> Result<Template, TemplatingError> {
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
    }
}

// Reformat author name to last name, first name if it has 2 names
fn reformat_author_name(author: &String) -> String {
    let parts = author.split(" ").collect::<Vec<&str>>();
    match parts.len() {
        0 => String::new(),
        2 => format!(
            "{}, {}",
            author.split(" ").collect::<Vec<&str>>()[1],
            author.split(" ").collect::<Vec<&str>>()[0]
        ),
        _ => author.clone(),
    }
}

// Substitute template with options
fn substitute_template(template: String, options: &Options) -> Result<String, TemplatingError> {
    let mut template = template;
    let cfg_dir = match dirs::config_dir() {
        Some(dir) => dir,
        None => return Err(TemplatingError::CouldNotFindCfgDir),
    };

    let lib_file = cfg_dir.join(format!("typstgen/lib.typ"));

    if lib_file.exists() {
        template = format!("#include {}\n\n {}", lib_file.display(), template);
    }

    // Get the author name, infer it if git inference is enabled
    let author = match options.author.clone() {
        Some(author) => author,
        None => {
            if options.name_inference {
                match realname() {
                    Ok(username) => {
                        if options.inferred_name_reformat {
                            reformat_author_name(&username)
                        } else {
                            username
                        }
                    }
                    Err(_) => AUTHOR_PLACEHOLDER.to_string(),
                }
            } else {
                AUTHOR_PLACEHOLDER.to_string()
            }
        }
    };

    // Substitute author name, reformatted to last name, first name
    template = template.replace("{{AUTHOR_NAME}}", &author);

    // Substitute author ORCID ID if it exists
    // The ORCID is only declared if an ORCID ID is provided
    match options.orcid.clone() {
        id => {
            if template.contains("{{ORCID_ICON_DECLARATION}}") {
                template = template.replace(
                    "{{ORCID_ID}}",
                    &format!(" #orcid_svg https://orcid.org/{}", id),
                );
                template = template.replace(
                    "{{ORCID_ICON_DECLARATION}}",
                    format!(
                        "#let orcid_svg = box(image(bytes(\"{}\"), width: {}pt, height: {}pt), height: {}pt)",
                        ORCID_IMAGE.replace("\"", "\\\""),
                        ORCID_ICON_SIZE_PT,
                        ORCID_ICON_SIZE_PT,
                        ORCID_ICON_SIZE_PT,
                    ).as_str(),
                );
            } else {
                template = template.replace("{{ORCID_ID}}", &id);
            }
        }
    }

    template = template.replace("{{LANG}}", &options.lang);

    Ok(template)
}

pub fn assemble_template(options: &Options) -> Result<String, TemplatingError> {
    let template = get_template(options.template.clone())?;
    let template_string = match template {
        Template::Article(content) => content,
        Template::Report(content) => content,
        Template::Custom(content) => content,
    };
    let final_string = substitute_template(template_string, &options)?;
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
