#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate human_panic;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate exitfailure;
extern crate issue_template;

use exitfailure::ExitFailure;
use issue_template::Cli;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
  setup_panic!();
  let args = Cli::from_args();
  args.log(env!("CARGO_PKG_NAME"))?;
  info!("program started");

  let dir = args.dir()?;
  let name = args.name()?;
  let templ = issue_template::Templates::new(dir, name)?;
  templ.write_all()?;
  Ok(())
}
