use mkdirp::mkdirp;
use std::path::PathBuf;

use failure::ResultExt;

/// GitHub template struct.
pub struct Templates {
  name: String,
  dir: PathBuf,
}

impl Templates {
  /// Create a new instance.
  pub fn new(mut dir: PathBuf, name: String) -> ::Result<Self> {
    dir.push(".github");
    mkdirp(&dir).context(::ErrorKind::Other);
    Ok(Self { name, dir })
  }
}
