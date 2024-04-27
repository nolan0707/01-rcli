use std::{fmt::Display, str::FromStr};

use super::verify_input_file;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    Decode(Base64DecodeOpts),
    Encode(Base64EncodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser=verify_input_file, default_value="-")]
    pub input: String,
    #[arg(long, value_parser=parse_base64_format,default_value="standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser=verify_input_file, default_value="-")]
    pub input: String,
    #[arg(long, value_parser=parse_base64_format,default_value="standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> anyhow::Result<Base64Format> {
    format.parse::<Base64Format>()
}

impl From<Base64Format> for &str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url_safe",
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "url_safe" => Ok(Base64Format::UrlSafe),
            v => anyhow::bail!("Unsupported format {}", v),
        }
    }
}
