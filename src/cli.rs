// The module that handles the CLI

use crate::Options;
use crate::templates::{TemplateSource, TemplatingError, get_template_source};
use clap::Parser;

pub enum CliError {
    TemplateError(TemplatingError),
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

pub fn parse_cli_args(args: Args) -> Result<Options, CliError> {
    const AUTHOR_NAME_PLACEHOLDER: &str = "AUTHOR NAME HERE";

    let template: TemplateSource = match args.template {
        Some(template) => match get_template_source(template) {
            Ok(template) => template,
            Err(error) => return Err(CliError::TemplateError(error)),
        },
        None => TemplateSource::DefaultTemplate,
    };

    Ok(Options {
        template: template,
        author: args.author,
        orcid: args.orcid,
        debug: args.debug,
    })
}
