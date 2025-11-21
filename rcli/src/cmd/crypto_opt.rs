use anyhow::{Result, bail};
use clap::Parser;

#[derive(Debug, Parser)]
pub enum CryptoOpts {
    #[command(name = "encode", about = "encode input to a method")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "decode input from a method")]
    Decode(DecodeOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum Method {
    All,
    Base64,
    SHA256,
    SHA1,
    MD5,
}

#[derive(Debug, Clone, Copy)]
pub enum InputType {
    String,
    File,
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(long, help = "input type", default_value = "str", value_parser = input_type_parse)]
    pub t: InputType,

    #[arg(long, short, help = "input content")]
    pub input: String,

    #[arg(long, short, help = "crypto in which method", default_value = "all", value_parser = method_parse)]
    pub method: Method,

    #[arg(long, help = "Support upper case?", default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub uppercase: bool,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    /// The input file to decode.
    #[clap(short, long)]
    pub input: String,

    #[arg(long, short, help = "crypto in which method", default_value = "all", value_parser = method_parse)]
    pub method: Method,
}

fn method_parse(s: &str) -> Result<Method> {
    match s {
        "base64" => Ok(Method::Base64),
        "sha1" => Ok(Method::SHA1),
        "sha256" => Ok(Method::SHA256),
        "md5" => Ok(Method::MD5),
        _ => Ok(Method::All),
    }
}

fn input_type_parse(s: &str) -> Result<InputType> {
    match s {
        "str" => Ok(InputType::String),
        "file" => Ok(InputType::File),
        _ => bail!("wrong input type"),
    }
}
