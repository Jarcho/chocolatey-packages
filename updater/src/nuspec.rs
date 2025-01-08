use crate::{Error, Result};
use bstr::ByteSlice;
use core::fmt::{self, Display, Formatter};
use core::ops::Range;
use core::str::{self, FromStr};
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
  pub major: u16,
  pub minor: u16,
  pub patch: u16,
  pub build: u16,
}
impl Display for Version {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}.{}", self.major, self.minor)?;
    match (self.patch, self.build) {
      (0, 0) => Ok(()),
      (_, 0) => write!(f, ".{}", self.patch),
      (_, _) => write!(f, "{}.{}", self.patch, self.build),
    }
  }
}
impl FromStr for Version {
  type Err = ();
  fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
    let mut parts = s.strip_prefix('v').unwrap_or(s).splitn(4, '.');
    Ok(Version {
      major: parts.next().ok_or(())?.parse().map_err(|_| ())?,
      minor: parts.next().ok_or(())?.parse().map_err(|_| ())?,
      patch: match parts.next() {
        Some(x) => x.parse().map_err(|_| ())?,
        None => 0,
      },
      build: match parts.next() {
        Some(x) => x.parse().map_err(|_| ())?,
        None => 0,
      },
    })
  }
}

/// An open and loaded nuspec file.
pub struct File {
  file: fs::File,
  data: Vec<u8>,
  version_range: Range<usize>,
  desc_range: Range<usize>,
  pub version: Version,
}
impl File {
  /// Opens and parses the nuspec file at the given path.
  pub fn open(path: &Path) -> Result<Self> {
    const VERSION_START: &[u8] = b"<version>";
    const VERSION_END: &[u8] = b"</version>";
    const DESC_START: &[u8] = b"<description>";
    const DESC_END: &[u8] = b"</description>";

    let mut file = fs::File::options()
      .read(true)
      .write(true)
      .open(path)
      .map_err(Error::NuspecIo)?;
    let meta = file.metadata().map_err(Error::NuspecIo)?;
    let mut data = Vec::with_capacity(meta.len() as usize + 1);
    file.read_to_end(&mut data).map_err(Error::NuspecIo)?;

    let version_start =
      data.find(VERSION_START).ok_or(Error::NuspecNoVersion)? + VERSION_START.len();
    let version_end = version_start
      + data[version_start..]
        .find(VERSION_END)
        .ok_or(Error::NuspecNoVersion)?;

    let version =
      str::from_utf8(&data[version_start..version_end]).map_err(|_| Error::NuspecNoVersion)?;
    let version = version.parse().map_err(|_| Error::NuspecNoVersion)?;

    let desc_start = data.find(DESC_START).ok_or(Error::NuspecNoDesc)? + DESC_START.len();
    let desc_end = desc_start + data[desc_start..].find(DESC_END).ok_or(Error::NuspecNoDesc)?;

    Ok(Self {
      file,
      data,
      version_range: version_start..version_end,
      desc_range: desc_start..desc_end,
      version,
    })
  }

  /// Updates, saves and closes the given nuspec file.
  pub fn save_updated(mut self, version: Version, description: Option<&str>) -> Result<()> {
    let mut buf = Vec::with_capacity(self.data.len() + 1024);
    if let Some(description) = description {
      if self.version_range.start < self.desc_range.start {
        buf.extend_from_slice(&self.data[..self.version_range.start]);
        let _ = write!(buf, "{version}");
        buf.extend_from_slice(&self.data[self.version_range.end..self.desc_range.start]);
        buf.extend_from_slice(description.as_bytes());
        buf.extend_from_slice(&self.data[self.desc_range.end..]);
      } else {
        buf.extend_from_slice(&self.data[..self.desc_range.start]);
        buf.extend_from_slice(description.as_bytes());
        buf.extend_from_slice(&self.data[self.desc_range.end..self.version_range.start]);
        let _ = write!(buf, "{version}");
        buf.extend_from_slice(&self.data[self.version_range.end..]);
      }
    } else {
      buf.extend_from_slice(&self.data[..self.version_range.start]);
      let _ = write!(buf, "{version}");
      buf.extend_from_slice(&self.data[self.version_range.end..]);
    }

    self.file.seek(SeekFrom::Start(0)).map_err(Error::NuspecIo)?;
    self.file.set_len(buf.len() as u64).map_err(Error::NuspecIo)?;
    self.file.write_all(&buf).map_err(Error::NuspecIo)
  }
}
