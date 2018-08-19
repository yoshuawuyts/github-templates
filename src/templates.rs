use failure::ResultExt;
use mkdirp::mkdirp;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

static BUG_REPORT: &str = include_str!("../templates/bug_report.md");

/// GitHub template struct.
pub struct Templates {
  name: String,
  dir: PathBuf,
}

impl Templates {
  /// Create a new instance. Creates a `.github` directory if it doesn't exist
  /// already.
  pub fn new(mut dir: PathBuf, name: String) -> ::Result<Self> {
    dir.push(".github");
    mkdirp(&dir).context(::ErrorKind::Other)?;
    Ok(Self { name, dir })
  }

  pub fn write_all(&self) -> ::Result<()> {
    let issue_dir = self.dir.join("ISSUE_TEMPLATE");
    mkdirp(&issue_dir).context(::ErrorKind::Other)?;

    let mut file = File::create(issue_dir.join("bug_report.md"))
      .context(::ErrorKind::Other)?;

    file
      .write_all(BUG_REPORT.as_bytes())
      .context(::ErrorKind::Other)?;

    Ok(())
  }
}
