use std::path::PathBuf;
use std::str::FromStr;
use structopt::{clap::Shell, StructOpt};

include!("src/lib.rs");

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  let outdir = ::std::env::var_os("OUT_DIR").expect("OUT_DIR not found.");
  let mut app = cli::Cli::clap();

  let target: PathBuf = outdir.into();
  let target = target.join("../../../scripts");
  mkdirp::mkdirp(&target)?;

  for shell in &Shell::variants() {
    let shell = Shell::from_str(*shell)?;
    app.gen_completions(env!("CARGO_PKG_NAME"), shell, &target);
  }

  Ok(())
}
