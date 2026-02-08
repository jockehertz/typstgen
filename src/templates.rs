// This module handles templating

use crate::Options;
use crate::defaults::{
    ARTICLE_TEMPLATE_STRING, ORCID_ICON_SIZE_PT, ORCID_IMAGE, REPORT_TEMPLATE_STRING,
    TEMPLATE_DIRECTORY,
};
use dirs;
use std::fs;
use std::path::{Path, PathBuf};

// An enum representing the different types of templates available
#[derive(Debug, Clone)]
pub enum Template {
    Report(String),
    Article(String),
    Custom(String),
}

// An enum representing the different sources of templates available
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateSource {
    BuiltinReport,
    BuiltinArticle,
    Custom(PathBuf),
}

// An enum representing the different errors that can occur during templating
#[derive(Debug, PartialEq)]
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

fn lib_file_exists(lib_file: impl AsRef<Path>) -> bool {
    match dirs::config_dir() {
        Some(path) => path.join("typstgen").join(lib_file).exists(),
        None => false,
    }
}

// Substitute ORCID icon declaration and ID into the template
fn substitute_orcid(template: &str, options: &Options) -> String {
    let return_template = if template.contains("{{ORCID_ICON_DECLARATION}}") {
        let orcid_icon = format!(
            "#let orcid_svg = box(image(bytes(\"{}\"), width: {}pt, height: {}pt), height: {}pt)",
            ORCID_IMAGE.replace("\"", "\\\""),
            ORCID_ICON_SIZE_PT,
            ORCID_ICON_SIZE_PT,
            ORCID_ICON_SIZE_PT,
        );
        template
            .replace(
                "{{ORCID_ID}}",
                format!(" #orcid_svg https://orcid.org/{}", &options.orcid).as_str(),
            )
            .replace("{{ORCID_ICON_DECLARATION}}", &orcid_icon)
    } else {
        template.replace(
            "{{ORCID_ID}}",
            format!(" | https://orcid.org/{}", &options.orcid).as_str(),
        )
    };

    return_template
}

// Substitute template with options
fn substitute_template(template: String, options: &Options) -> Result<String, TemplatingError> {
    let lib_file = options.lib_file.clone();

    let template = if lib_file_exists(&lib_file) {
        format!("#import \"{}\": *\n\n {}", lib_file.display(), template)
    } else {
        template
    };

    // Substitute author name, reformatted to last name, first name
    let template = template.replace("{{AUTHOR_NAME}}", &options.author);

    // Substitute author ORCID ID if it exists
    // The ORCID is only declared if an ORCID ID is provided
    let template = substitute_orcid(&template, &options);

    let template = template.replace("{{LANG}}", &options.lang);

    let template = template.replace("{{EMAIL}}", &options.email);

    Ok(template)
}

// Assemble the template by substituting variables and importing the library file
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
pub fn get_template_source(template_name: &str) -> Result<TemplateSource, TemplatingError> {
    match template_name {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_substitute_orcid() {
        let template = String::from("{{ORCID_ID}}");
        let options = Options {
            output: String::from("output"),
            template: TemplateSource::Custom(PathBuf::from("custom_template.typ")),
            author: String::from("John Doe"),
            email: String::from("john.doe@example.com"),
            lang: String::from("en"),
            debug: false,
            lib_file: PathBuf::from("lib.typ"),
            orcid: String::from("0000-0002-1825-0097"),
        };
        let result = substitute_orcid(&template, &options);
        assert_eq!(
            result,
            String::from(" | https://orcid.org/0000-0002-1825-0097")
        );
    }

    #[test]
    fn test_substitute_author() {
        let template = String::from("{{AUTHOR_NAME}}");
        let options = Options {
            output: String::from("output"),
            template: TemplateSource::Custom(PathBuf::from("custom_template.typ")),
            author: String::from("John Doe"),
            email: String::from("john.doe@example.com"),
            lang: String::from("en"),
            debug: false,
            lib_file: PathBuf::from("lib.typ"),
            orcid: String::from("0000-0002-1825-0097"),
        };
        let result = substitute_template(template, &options).ok().unwrap();
        assert!(result.contains("John Doe"));
        assert!(!result.contains("{{AUTHOR_NAME}}"));
    }

    #[test]
    fn test_substitute_email() {
        let template = String::from("{{EMAIL}}");
        let options = Options {
            output: String::from("output"),
            template: TemplateSource::Custom(PathBuf::from("custom_template.typ")),
            author: String::from("John Doe"),
            email: String::from("john.doe@example.com"),
            lang: String::from("en"),
            debug: false,
            lib_file: PathBuf::from("lib.typ"),
            orcid: String::from("0000-0002-1825-0097"),
        };
        let result = substitute_template(template, &options).ok().unwrap();
        assert!(result.contains("john.doe@example.com"));
        assert!(!result.contains("{{EMAIL}}"));
    }

    #[test]
    fn test_substitute_lang() {
        let template = String::from("{{LANG}}");
        let options = Options {
            output: String::from("output"),
            template: TemplateSource::Custom(PathBuf::from("custom_template.typ")),
            author: String::from("John Doe"),
            email: String::from("john.doe@example.com"),
            lang: String::from("en"),
            debug: false,
            lib_file: PathBuf::from("lib.typ"),
            orcid: String::from("0000-0002-1825-0097"),
        };
        let result = substitute_template(template, &options).ok().unwrap();
        assert!(result.contains("en"));
        assert!(!result.contains("{{LANG}}"));
    }

    #[test]
    fn test_substitute_debug() {
        let template = String::from("{{LANG}}");
        let options = Options {
            output: String::from("output"),
            template: TemplateSource::Custom(PathBuf::from("custom_template.typ")),
            author: String::from("John Doe"),
            email: String::from("john.doe@example.com"),
            lang: String::from("en"),
            debug: true,
            lib_file: PathBuf::from("lib.typ"),
            orcid: String::from("0000-0002-1825-0097"),
        };
        let result = substitute_template(template, &options).ok().unwrap();
        assert!(result.contains("en"));
        assert!(!result.contains("{{LANG}}"));
    }

    #[test]
    fn test_substitute_author_and_email() {
        let template = String::from("{{AUTHOR_NAME}} {{EMAIL}}");
        let options = Options {
            output: String::from("output"),
            template: TemplateSource::Custom(PathBuf::from("custom_template.typ")),
            author: String::from("John Doe"),
            email: String::from("john.doe@example.com"),
            lang: String::from("en"),
            debug: false,
            lib_file: PathBuf::from("lib.typ"),
            orcid: String::from("0000-0002-1825-0097"),
        };
        let result = substitute_template(template, &options).ok().unwrap();
        assert!(result.contains("John Doe john.doe@example.com"));
        assert!(!result.contains("{{AUTHOR_NAME}}"));
        assert!(!result.contains("{{EMAIL}}"));
    }

    #[test]
    fn test_substitute_orcid_and_author() {
        let template = String::from("{{AUTHOR_NAME}}{{ORCID_ID}}");
        let options = Options {
            output: String::from("output"),
            template: TemplateSource::Custom(PathBuf::from("custom_template.typ")),
            author: String::from("John Doe"),
            email: String::from("john.doe@example.com"),
            lang: String::from("en"),
            debug: false,
            lib_file: PathBuf::from("lib.typ"),
            orcid: String::from("0000-0002-1825-0097"),
        };
        let result = substitute_template(template, &options).ok().unwrap();
        assert!(result.contains("John Doe | https://orcid.org/0000-0002-1825-0097"));
        assert!(!result.contains("{{ORCID_ID}}"));
        assert!(!result.contains("{{AUTHOR_NAME}}"));
    }
}
