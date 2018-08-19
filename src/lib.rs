#[macro_use]
extern crate structopt;
extern crate clap_flags;
#[macro_use]
extern crate failure;
extern crate mkdirp;

mod cli;
mod error;
mod templates;

pub use cli::Cli;
pub use error::{Error, ErrorKind, Result};
pub use templates::Templates;
