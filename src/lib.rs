#[macro_use]
extern crate structopt;
extern crate clap_flags;
#[macro_use]
extern crate failure;

mod cli;
mod error;

pub use cli::Cli;
pub use error::{Error, ErrorKind, Result};
