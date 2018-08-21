#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate human_panic;
extern crate exitfailure;
extern crate github_templates;
extern crate log;
extern crate structopt;

use exitfailure::ExitFailure;
use github_templates::Cli;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
  setup_panic!();
  let args = Cli::from_args();
  args.log(env!("CARGO_PKG_NAME"))?;
  let dir = args.dir()?;
  let name = args.name()?;
  let templ = github_templates::Templates::new(dir, name)?;
  templ.write_all()?;
  Ok(())
}
