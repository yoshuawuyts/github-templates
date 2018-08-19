use failure::ResultExt;
use mkdirp::mkdirp;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

static BUG_REPORT: &str = include_str!("../templates/bug_report.md");
static QUESTION: &str = include_str!("../templates/question.md");
static FEATURE_REQUEST: &str = include_str!("../templates/feature_request.md");

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

    self.write("bug_report.md", BUG_REPORT)?;
    self.write("question.md", QUESTION)?;
    self.write("feature_request.md", FEATURE_REQUEST)?;

    Ok(())
  }

  fn write(&self, file_name: &str, template: &str) -> ::Result<()> {
    let issue_dir = self.dir.join("ISSUE_TEMPLATE");
    let mut file =
      File::create(issue_dir.join(file_name)).context(::ErrorKind::Other)?;

    let bug_report = str::replace(template, "{{Project}}", &self.name);
    file
      .write_all(bug_report.as_bytes())
      .context(::ErrorKind::Other)?;
    Ok(())
  }
}
