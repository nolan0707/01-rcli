mod cli;
mod process;

pub use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand, GenPassOpts, Opts, SubCommand,
};
pub use process::{process_csv, process_decode, process_encode, process_genpass};
