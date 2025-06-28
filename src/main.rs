mod mappings;

use std::fmt::Display;

use clap::{Parser, ValueEnum, command};
use mappings::{MappingType, to_html};

#[derive(Debug, Clone, Default, ValueEnum)]
/// Possible formats for output
enum ConversionFormat {
    /// Just vector
    #[default]
    Vec,
    /// Html
    Html,
}

impl Display for ConversionFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionFormat::Vec => write!(f, "vec"),
            ConversionFormat::Html => write!(f, "html"),
        }
    }
}

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
/// CLI tool for converting words to binary format and mapping it to specific links
struct Cli {
    /// Number of times to greet
    #[arg(short, long, default_value_t = ConversionFormat::Vec)]
    format: ConversionFormat,

    /// Mapping type
    #[arg(short, long, default_value_t = MappingType::R34)]
    map: MappingType,

    /// Word to convert
    word: String,
}

fn main() {
    let cli = Cli::parse();
    match cli.format {
        ConversionFormat::Vec => {
            println!(
                "{:#?}",
                cli.map.to_url(cli.word.chars()).collect::<Vec<_>>()
            )
        }
        ConversionFormat::Html => {
            println!("{}", to_html(cli.map.to_url(cli.word.chars())))
        }
    }
}
