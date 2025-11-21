use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, help = "Length of password", default_value_t = 16)]
    pub length: u8,

    #[arg(long, help = "Support upper case?", default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub uppercase: bool,

    #[arg(long, help = "Support lower case?", default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub lowercase: bool,

    #[arg(long, help = "Support number?", default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub number: bool,

    #[arg(long, help = "Support symbol?", default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub symbol: bool,
}