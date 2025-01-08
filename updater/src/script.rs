use crate::download::Sha1;
use crate::{Error, Result};
use bstr::ByteSlice;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

/// The set of variables to replace in an install script.
#[derive(Default)]
pub struct Replacements<'a> {
  pub url32: Option<&'a str>,
  pub checksum32: Option<Sha1>,
  pub url64: Option<&'a str>,
  pub checksum64: Option<Sha1>,
}
impl Replacements<'_> {
  fn ensure_empty(&self) -> Result<()> {
    if self.url32.is_some() {
      Err(Error::ScriptMissingVar("url32"))
    } else if self.url64.is_some() {
      Err(Error::ScriptMissingVar("url64"))
    } else if self.checksum32.is_some() {
      Err(Error::ScriptMissingVar("checksum32"))
    } else if self.checksum64.is_some() {
      Err(Error::ScriptMissingVar("checksum64"))
    } else {
      Ok(())
    }
  }

  fn write_replacement(&mut self, var: &[u8], dst: &mut Vec<u8>) -> bool {
    match (var, &*self) {
      (b"url32", &Replacements { url32: Some(x), .. }) => {
        let _ = writeln!(dst, "$url32      = '{x}'");
        self.url32 = None;
        true
      }
      (b"url64", &Replacements { url64: Some(x), .. }) => {
        let _ = writeln!(dst, "$url64      = '{x}'");
        self.url64 = None;
        true
      }
      (b"checksum32", Replacements { checksum32: Some(x), .. }) => {
        let _ = writeln!(dst, "$checksum32 = '{x}'");
        self.checksum32 = None;
        true
      }
      (b"checksum64", Replacements { checksum64: Some(x), .. }) => {
        let _ = writeln!(dst, "$checksum64 = '{x}'");
        self.checksum64 = None;
        true
      }
      _ => false,
    }
  }

  /// Applies the replacements to the given install script.
  pub fn apply(mut self, path: &Path) -> Result<()> {
    let mut file = File::options()
      .read(true)
      .write(true)
      .open(path)
      .map_err(Error::ScriptIo)?;
    let meta = file.metadata().map_err(Error::ScriptIo)?;
    let mut data = Vec::with_capacity(meta.len() as usize + 1);
    file.read_to_end(&mut data).map_err(Error::ScriptIo)?;

    let mut new_data = Vec::with_capacity(data.len() + 100);
    for line in data.lines_with_terminator() {
      let updated = line
        .split_once_str(b"=")
        .and_then(|(var, _)| var.strip_prefix(b"$"))
        .map(|var| self.write_replacement(var.trim_end(), &mut new_data))
        .unwrap_or(false);
      if !updated {
        new_data.extend_from_slice(line);
      }
    }
    self.ensure_empty()?;

    file.seek(SeekFrom::Start(0)).map_err(Error::ScriptIo)?;
    file.set_len(new_data.len() as u64).map_err(Error::ScriptIo)?;
    file.write_all(&new_data).map_err(Error::ScriptIo)
  }
}
