use crate::cmd::crypto_opt::{InputType, Method};
use anyhow::{Error, bail};
use base64::engine::general_purpose::STANDARD;
use base64::{DecodeError, Engine};
use md5::Md5;
use sha1::Sha1;
use sha2::{Digest, Sha256};

macro_rules! print_result {
    ($label:expr, $value:expr, $uppercase:expr) => {
        if $uppercase {
            println!("{}:{}", $label, $value.to_uppercase());
        } else {
            println!("{}:{}", $label, $value);
        }
    };
}

pub fn process_decode(input: &str, method: Method) -> anyhow::Result<()> {
    match method {
        Method::Base64 => {
            println!("base64:{}", decode_base64(&input.as_bytes().to_vec())?);
        }
        _ => {
            bail!("don't support other method")
        }
    }
    Ok(())
}

// pub fn process_encode_file(input: &str, ) -> String {
// }

/// crypto input by method
pub fn process_encode(input: &str, input_type: InputType, method: Method, uppercase: bool) -> anyhow::Result<()> {

    let input = match input_type {
        InputType::File => {
            let file = std::fs::read(input)?;
            file
        }
        InputType::String => input.as_bytes().to_vec(),
    };

    let input = &input;

    match method {
        Method::All => {
            print_result!("base64", encode_base64(input), uppercase);
            print_result!("sha256", encode_sha256(input), uppercase);
            print_result!("sha1  ", encode_sha1(input), uppercase);
            print_result!("md5   ", encode_md5(input), uppercase);
        }
        Method::Base64 => {
            print_result!("base64", encode_base64(input), uppercase);
        }

        Method::SHA256 => {
            print_result!("sha256", encode_sha256(input), uppercase);
        }
        Method::SHA1 => {
            print_result!("sha1  ", encode_sha1(input), uppercase);
        }
        Method::MD5 => {
            print_result!("md5   ", encode_md5(input), uppercase);
        }
    }

    Ok(())
}

fn encode_base64(input: &Vec<u8>) -> String {
    STANDARD.encode(input)
}

fn decode_base64(input: &Vec<u8>) -> anyhow::Result<String> {
    let result = STANDARD.decode(input)?;
    Ok(String::from_utf8(result)?)
}

/// encode input sha256
fn encode_sha256(input: &Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(&result)
}

fn encode_sha1(input: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(&result)
}

fn encode_md5(input: &Vec<u8>) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(&result)
}
