// The module that handles the CLI

use crate::Options;
use crate::templates::{DEFAULT_TEMPLATE, TemplateSource, TemplatingError, get_template_source};
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

// Parse the CLI arguments into an Options struct
pub fn parse_cli_args(args: Args) -> Result<Options, CliError> {
    let template: TemplateSource = match args.template {
        Some(template) => match get_template_source(template) {
            Ok(template) => template,
            Err(error) => return Err(CliError::TemplateError(error)),
        },
        None => TemplateSource::DefaultTemplate,
    };

    Ok(Options {
        output: args.output,
        template: template,
        author: args.author,
        orcid: args.orcid,
        lang: args.lang.trim().to_string(),
        default_template: DEFAULT_TEMPLATE,
        debug: args.debug,
    })
}
