use crate::templates::TemplateSource;

// The default template to be used
pub const DEFAULT_TEMPLATE: TemplateSource = TemplateSource::BuiltinReport;
pub const DEFAULT_OUTPUT: &str = "output";

// This directory is relative to the home directory.
pub const TEMPLATE_DIRECTORY: &str = "typstgen/templates";

pub const REPORT_TEMPLATE_STRING: &str = include_str!("./templates/builtin_report.typ");
pub const ARTICLE_TEMPLATE_STRING: &str = include_str!("./templates/builtin_article.typ");

pub const DEFAULT_LANG: &str = "en";
pub const AUTHOR_PLACEHOLDER: &str = "Unknown Author";
pub const DEFAULT_EMAIL: &str = "unknown@example.com";
pub const NAME_INFERENCE_DEFAULT: bool = true;

// Include the ORCID SVG icon as a string
pub const ORCID_IMAGE: &str = include_str!("assets/orcid.svg");
pub const ORCID_ICON_SIZE_PT: f64 = 18.0;
pub const DEFAULT_ORCID: &str = "0000-0000-0000-0000";

pub const DEFAULT_LIB_FILE: &str = "lib.typ";
