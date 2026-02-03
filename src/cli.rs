// The module that handles the CLI

use crate::{
    defaults::DEFAULT_LANG,
    templates::{TemplateSource, TemplatingError, get_template_source},
};
use clap::Parser;

// This enum represents the different types of errors that can occur during CLI parsing
// It is currently only with one variant, I expect that more will be added in the future
pub enum CliError {
    TemplateError(TemplatingError),
}

// Implement the From trait for converting TemplatingError to CliError
impl From<TemplatingError> for CliError {
    fn from(error: TemplatingError) -> Self {
        CliError::TemplateError(error)
    }
}

// The struct for the CLI arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    output: String,
    template: Option<String>,

    // Flags below this
    // Author flag
    #[arg(short, long)]
    author: Option<String>,
    // ORCID flag
    #[arg(long, default_missing_value = "XXXX-XXXX-XXXX-XXXX")]
    orcid: Option<String>,
    // Language flag
    #[arg(short, long, default_value = DEFAULT_LANG)]
    lang: String,
    #[arg(short, long)]
    debug: bool,
}

#[derive(Debug)]
pub struct FlagOptions {
    pub output: String,
    pub template: TemplateSource,
    pub author: Option<String>,
    pub orcid: Option<String>,
    pub lang: String,
    pub debug: bool,
}
// Parse the CLI arguments into an Options struct
pub fn parse_cli_args(args: Args) -> Result<FlagOptions, CliError> {
    let template: TemplateSource = match args.template {
        Some(template) => get_template_source(template)?,
        None => TemplateSource::DefaultTemplate,
    };

    Ok(FlagOptions {
        output: args.output,
        template: template,
        author: args.author,
        orcid: args.orcid,
        lang: args.lang.trim().to_string(),
        debug: args.debug,
    })
}
