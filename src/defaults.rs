use crate::templates::TemplateSource;

// The default template to be used
pub const DEFAULT_TEMPLATE: TemplateSource = TemplateSource::BuiltinReport;

// This directory is relative to the home directory.
pub const TEMPLATE_DIRECTORY: &str = ".typstgen/templates";

pub const REPORT_TEMPLATE_STRING: &str = include_str!("./templates/builtin_report.typ");
pub const ARTICLE_TEMPLATE_STRING: &str = include_str!("./templates/builtin_article.typ");
