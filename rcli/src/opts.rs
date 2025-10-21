use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

    #[arg(short, long, help = "Output file", default_value = "output.json")]
    pub output: String,

    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,

    #[arg(
        short = 'H',
        long,
        help = "CSV has header or not",
        default_value_t = true
    )]
    pub header: bool,
}


pub fn input_parse(file_name: &str) -> Result<String, String> {
    if file_name.is_empty() {
        return Err("Input file cannot be empty".to_string());
    }

    if !Path::new(file_name).exists() {
        return Err("Input file doesn't exist".to_string());
    }

    Ok(file_name.to_string())
}