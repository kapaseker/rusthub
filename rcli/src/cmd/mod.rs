pub mod crypto_opt;
pub mod csv_opt;
pub mod gen_pass_opt;

use crate::cmd::crypto_opt::CryptoOpts;
use crate::cmd::csv_opt::CsvOpts;
use crate::cmd::gen_pass_opt::GenPassOpts;
use clap::Parser;
use std::fmt::Display;

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

    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Crypto(CryptoOpts),
}
