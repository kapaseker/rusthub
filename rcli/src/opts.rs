use clap::Parser;
use std::fmt;
use std::fmt::Display;
use std::path::Path;

/// Output formats: Json, Yaml, Toml
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

///
/// rcli csv -i input.csv -o output.json --header -d ','
///
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about)]
pub struct Opt {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file", value_parser = input_parse)]
    pub input: String,

    #[arg(short, long, help = "Output file")]
    pub output: Option<String>,

    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,

    #[arg(short, long, help = "output format", default_value = "json", value_parser = format_parse)]
    pub format: OutputFormat,

    #[arg(
        short = 'H',
        long,
        help = "CSV has header or not",
        default_value_t = true
    )]
    pub header: bool,
}

impl TryFrom<&str> for OutputFormat {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            v => Err(format!("Invalid format: {}", v)),
        }
    }
}

// impl From<OutputFormat> for &str {
//     fn from(value: OutputFormat) -> Self {
//         match value {
//             OutputFormat::Json => "json",
//             OutputFormat::Yaml => "yaml",
//             OutputFormat::Toml => "toml",
//         }
//     }
// }

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Toml => write!(f, "toml"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}

fn format_parse(format: &str) -> Result<OutputFormat, String> {
    return format.try_into();
}

fn input_parse(file_name: &str) -> Result<String, String> {
    if file_name.is_empty() {
        return Err("Input file cannot be empty".to_string());
    }

    if !Path::new(file_name).exists() {
        return Err("Input file doesn't exist".to_string());
    }

    Ok(file_name.to_string())
}
