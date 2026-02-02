// The module that handles the CLI

use crate::{
    defaults::DEFAULT_TEMPLATE,
    templates::{TemplateSource, TemplatingError, get_template_source},
};
use clap::Parser;

pub enum CliError {
    TemplateError(TemplatingError),
}

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
    #[arg(long)]
    orcid: Option<String>,
    // Language flag
    #[arg(short, long, default_value = "en")]
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
