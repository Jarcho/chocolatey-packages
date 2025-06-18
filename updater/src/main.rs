/// Running external commands. e.g. `git` and `choco`
mod command;
/// Downloading and verifying files.
mod download;
/// Interacting with the github api.
mod github;
/// Loading and updating the nuspec files.
mod nuspec;
/// Loading and updating the install scripts.
mod script;

use crate::{download::Sha1, nuspec::Version};
use core::{
  fmt::{self, Display, Formatter, Write},
  str,
};
use pgp::{Deserializable, SignedPublicKey, StandaloneSignature};
use std::{
  env, fs, io,
  path::{Path, PathBuf},
  process::{exit, ExitStatus},
  sync::OnceLock,
  thread,
  time::Duration,
};

enum Error {
  /// An IO error while manipulating a nuspec file.
  NuspecIo(io::Error),
  /// A nuspec file was missing the version tag.
  NuspecNoVersion,
  /// A nuspec file was missing the description tag.
  NuspecNoDesc,
  /// An http error while checking for a new version.
  UpdateHttp(Box<ureq::Error>),
  /// An io error while checking for a new version.
  UpdateIo(io::Error),
  /// No release information was found while checking for a new version.
  UpdateNoRelease,
  /// Failed to parse the version information while checking for a new version.
  UpdateParseVersion,
  /// An http error while fetching an asset.
  AssetHttp(&'static str, Box<ureq::Error>),
  /// An io error while fetching an asset.
  AssetIo(&'static str, io::Error),
  /// An expected asset was missing from the release.
  AssetMissing(&'static str),
  /// A duplicate asset was found with the release.
  AssetDuplicate(&'static str),
  /// An asset failed the pgp signature check.
  AssetSigFailed(&'static str),
  /// An asset did not have the expected checksum.
  AssetChecksumMismatch(&'static str),
  /// An asset was retrieved, but did not match the expected format.
  AssetBadFormat(&'static str, Box<dyn 'static + Send + Display>),
  /// An io error while running a command.
  CommandIo(&'static str, io::Error),
  /// A command failed with an unknown error.
  CommandStatus(&'static str, ExitStatus),
  /// An error occurred while pushing the updated git package.
  PushFailed,
  /// An io error while updating the install script.
  ScriptIo(io::Error),
  /// The install script was missing an expected variable.
  ScriptMissingVar(&'static str),
}
impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::NuspecIo(e) => write!(f, "error with nuspec file: {e}"),
      Self::NuspecNoVersion => f.write_str("error with nuspec file: version tag missing"),
      Self::NuspecNoDesc => f.write_str("error with nuspec file: description tag missing"),
      Self::UpdateHttp(e) => write!(f, "error fetching version: {e}"),
      Self::UpdateIo(e) => write!(f, "error fetching version: {e}"),
      Self::UpdateNoRelease => f.write_str("error fetching version: no release found"),
      Self::UpdateParseVersion => f.write_str("error fetching version: can't parse version"),
      Self::AssetHttp(asset, e) => write!(f, "error fetching asset `{asset}`: {e}"),
      Self::AssetIo(asset, e) => write!(f, "error fetching asset `{asset}`: {e}"),
      Self::AssetMissing(asset) => write!(f, "error fetching asset `{asset}`: not found"),
      Self::AssetDuplicate(asset) => {
        write!(f, "error fetching asset `{asset}`: multiple options found")
      }
      Self::AssetSigFailed(asset) => write!(
        f,
        "error fetching asset `{asset}`: signature verification failed"
      ),
      Self::AssetChecksumMismatch(asset) => write!(
        f,
        "error fetching asset `{asset}`: checksum verification failed"
      ),
      Self::AssetBadFormat(asset, e) => {
        write!(f, "error fetching asset `{asset}`: bad asset format: {e}")
      }
      Self::CommandIo(name, e) => write!(f, "error running `{name}`: {e}"),
      Self::CommandStatus(name, status) => write!(
        f,
        "error running `{name}`: process exited unsuccessfully: {status}"
      ),
      Self::PushFailed => f.write_str("error pushing package to git"),
      Self::ScriptIo(e) => write!(f, "error updating script: {e}"),
      Self::ScriptMissingVar(var) => {
        write!(f, "error updating script: missing variable `{var}`")
      }
    }
  }
}
type Result<T> = core::result::Result<T, Error>;

struct PlatformUpdate {
  url: String,
  checksum: Option<Sha1>,
  sig: Option<StandaloneSignature>,
}
impl PlatformUpdate {
  fn new(url: String) -> Self {
    Self { url, checksum: None, sig: None }
  }

  fn with_checksum(url: String, checksum: Sha1) -> Self {
    Self { url, checksum: Some(checksum), sig: None }
  }

  fn with_sig(url: String, sig: StandaloneSignature) -> Self {
    Self { url, checksum: None, sig: Some(sig) }
  }

  fn get_script_replacements(
    &self,
    agent: &ureq::Agent,
    name: &'static str,
    package: &Package,
  ) -> Result<(&str, Sha1)> {
    let checksum = download::checksum_and_verify_file(
      agent,
      name,
      &self.url,
      package.download_headers,
      self.checksum.as_ref(),
      match (package.get_sig_key, &self.sig) {
        (Some(get_sig_key), Some(sig)) => Some((get_sig_key(), sig)),
        (None, None) => None,
        (Some(_), None) => panic!(
          "download for package `{}` is missing a signature",
          package.name
        ),
        (None, Some(_)) => panic!("package `{}` is missing a pgp key", package.name),
      },
    )?;
    Ok((&self.url, checksum))
  }
}

struct PackageUpdate {
  x32: Option<PlatformUpdate>,
  x64: Option<PlatformUpdate>,
  version: Version,
}

enum UpdateStatus {
  UpToDate(Version),
  Updated { prev: Version, new: Version },
}

struct Package {
  name: &'static str,
  version_url: &'static str,
  download_headers: &'static [(&'static str, &'static str)],
  get_sig_key: Option<fn() -> &'static SignedPublicKey>,
  fetch_update: fn(&ureq::Agent, Version, ureq::Response) -> Result<Option<PackageUpdate>>,
}
impl Package {
  fn process(&self, agent: &ureq::Agent) -> Result<UpdateStatus> {
    let nuspec = nuspec::File::open(&Path::new(self.name).join(format!("{}.nuspec", self.name)))?;
    if let Some(update) = (self.fetch_update)(
      agent,
      nuspec.version,
      agent
        .get(self.version_url)
        .call()
        .map_err(|e| Error::UpdateHttp(Box::new(e)))?,
    )? {
      let mut replacements = script::Replacements::default();
      if let Some(platform) = &update.x32 {
        let (url, checksum) = platform.get_script_replacements(agent, "x86", self)?;
        replacements.url32 = Some(url);
        replacements.checksum32 = Some(checksum);
      }
      if let Some(platform) = &update.x64 {
        let (url, checksum) = platform.get_script_replacements(agent, "x64", self)?;
        replacements.url64 = Some(url);
        replacements.checksum64 = Some(checksum);
      }
      replacements.apply(&PathBuf::from_iter([
        Path::new(self.name),
        "tools".as_ref(),
        "chocolateyinstall.ps1".as_ref(),
      ]))?;
      let prev_version = nuspec.version;
      nuspec.save_updated(update.version, None)?;

      Ok(UpdateStatus::Updated { prev: prev_version, new: update.version })
    } else {
      Ok(UpdateStatus::UpToDate(nuspec.version))
    }
  }
}

static PACKAGES: &[Package] = &[
  Package {
    name: "assfiltermod",
    version_url: "https://api.github.com/repos/Blitzker/assfiltermod/releases",
    download_headers: &[],
    get_sig_key: None,
    fetch_update: |_, prev_version, response| {
      let release: github::Release = response.into_json().map_err(Error::UpdateIo)?;
      let version = release.tag.parse().map_err(|_| Error::UpdateParseVersion)?;
      if version > prev_version {
        let [url32, url64] = github::extract_assets(
          release.assets,
          &["x86", "x64"],
          (
            |x: &str| x.ends_with("_x32.zip"),
            |x: &str| x.ends_with("_x64.zip"),
          ),
        )?;
        Ok(Some(PackageUpdate {
          version,
          x32: Some(PlatformUpdate::new(url32)),
          x64: Some(PlatformUpdate::new(url64)),
        }))
      } else {
        Ok(None)
      }
    },
  },
  Package {
    name: "imhex",
    version_url: "https://api.github.com/repos/WerWolv/ImHex/releases",
    download_headers: &[],
    get_sig_key: None,
    fetch_update: |_, prev_version, response| {
      let release: github::Release = response.into_json().map_err(Error::UpdateIo)?;
      let version = release.tag.parse().map_err(|_| Error::UpdateParseVersion)?;
      if version > prev_version {
        let [url64] = github::extract_assets(release.assets, &["x64"], |x: &str| {
          x.ends_with("Windows-x86_64.msi")
        })?;
        Ok(Some(PackageUpdate {
          version,
          x32: None,
          x64: Some(PlatformUpdate::new(url64)),
        }))
      } else {
        Ok(None)
      }
    },
  },
  Package {
    name: "jujutsu.portable",
    version_url: "https://api.github.com/repos/jj-vcs/jj/releases",
    download_headers: &[],
    get_sig_key: None,
    fetch_update: |_, prev_version, response| {
      let release: github::Release = response.into_json().map_err(Error::UpdateIo)?;
      let version = release.tag.parse().map_err(|_| Error::UpdateParseVersion)?;
      if version > prev_version {
        let [url64] = github::extract_assets(release.assets, &["x64"], |x: &str| {
          x.ends_with("-x86_64-pc-windows-msvc.zip ")
        })?;
        Ok(Some(PackageUpdate {
          version,
          x32: None,
          x64: Some(PlatformUpdate::new(url64)),
        }))
      } else {
        Ok(None)
      }
    },
  },
  Package {
    name: "madvr",
    version_url: "https://www.videohelp.com/software/madVR",
    download_headers: &[("Referrer", "https://www.videohelp.com/software/madVR")],
    get_sig_key: None,
    fetch_update: |_, prev_version, response| {
      static SEARCH_TEXT: &str = r#"href="https://www.videohelp.com/download/madVR"#;
      let text = response.into_string().map_err(Error::UpdateIo)?;
      let pos = text.find(SEARCH_TEXT).ok_or(Error::UpdateNoRelease)?;
      let version_start = pos + SEARCH_TEXT.len();
      let version = Version {
        major: text
          .get(version_start..version_start + 1)
          .ok_or(Error::UpdateNoRelease)?
          .parse()
          .map_err(|_| Error::UpdateNoRelease)?,
        minor: text
          .get(version_start + 1..version_start + 3)
          .ok_or(Error::UpdateNoRelease)?
          .parse()
          .map_err(|_| Error::UpdateNoRelease)?,
        patch: text
          .get(version_start + 3..version_start + 5)
          .ok_or(Error::UpdateNoRelease)?
          .parse()
          .map_err(|_| Error::UpdateNoRelease)?,
        build: 0,
      };
      if text
        .get(version_start + 5..version_start + 10)
        .ok_or(Error::UpdateNoRelease)?
        != ".zip\""
      {
        return Err(Error::UpdateNoRelease);
      }
      if version > prev_version {
        Ok(Some(PackageUpdate {
          version,
          x32: None,
          x64: Some(PlatformUpdate::new(text[pos..version_start + 9].to_owned())),
        }))
      } else {
        Ok(None)
      }
    },
  },
  Package {
    name: "nextdns-cli",
    version_url: "https://api.github.com/repos/nextdns/nextdns/releases",
    download_headers: &[],
    get_sig_key: None,
    fetch_update: |agent, prev_version, response| {
      let release: github::Release = response.into_json().map_err(Error::UpdateIo)?;
      let version = release.tag.parse().map_err(|_| Error::UpdateParseVersion)?;
      if version > prev_version {
        let [url32, url64, checksums] = github::extract_assets(
          release.assets,
          &["x86", "x64", "checksums"],
          (
            |x: &str| x.ends_with("_windows_386.zip"),
            |x: &str| x.ends_with("_windows_amd64.zip"),
            |x: &str| x == "checksums.txt",
          ),
        )?;
        let checksums = download::download_file(agent, "checksums", &checksums)?;
        let checksums = String::from_utf8(checksums)
          .map_err(|e| Error::AssetBadFormat("checksums", Box::new(e)))?;

        let mut checksum32 = None;
        let mut checksum64 = None;
        for line in checksums.lines() {
          if line.ends_with("_windows_386.zip") {
            checksum32 = line.get(..64).and_then(|x| x.parse().ok());
          } else if line.ends_with("_windows_amd64.zip") {
            checksum64 = line.get(..64).and_then(|x| x.parse().ok());
          }
        }
        let (Some(checksum32), Some(checksum64)) = (checksum32, checksum64) else {
          return Err(Error::AssetBadFormat(
            "checksums",
            Box::new("missing entries"),
          ));
        };
        Ok(Some(PackageUpdate {
          version,
          x32: Some(PlatformUpdate::with_checksum(url32, checksum32)),
          x64: Some(PlatformUpdate::with_checksum(url64, checksum64)),
        }))
      } else {
        Ok(None)
      }
    },
  },
  Package {
    name: "protonmailbridge",
    version_url: "https://api.github.com/repos/ProtonMail/proton-bridge/releases",
    download_headers: &[],
    get_sig_key: Some(|| {
      static KEY: OnceLock<SignedPublicKey> = OnceLock::new();
      KEY.get_or_init(|| {
        SignedPublicKey::from_string(include_str!("bridge_pubkey.gpg"))
          .unwrap()
          .0
      })
    }),
    fetch_update: |agent, prev_version, response| {
      let release: github::Release = response.into_json().map_err(Error::UpdateIo)?;
      let version = release.tag.parse().map_err(|_| Error::UpdateParseVersion)?;
      if version > prev_version {
        let [url64, sig64] = github::extract_assets(
          release.assets,
          &["x64", "x64-sig"],
          (
            |x: &str| x == "Bridge-Installer.exe",
            |x: &str| x == "Bridge-Installer.exe.sig",
          ),
        )?;
        let sig64 = download::download_file(agent, "x64-sig", &sig64)?;
        let sig64 = StandaloneSignature::from_bytes(&*sig64)
          .map_err(|e| Error::AssetBadFormat("x64-sig", Box::new(e)))?;
        Ok(Some(PackageUpdate {
          version,
          x32: None,
          x64: Some(PlatformUpdate::with_sig(url64, sig64)),
        }))
      } else {
        Ok(None)
      }
    },
  },
  Package {
    name: "streamwhatyouhear",
    version_url: "https://api.github.com/repos/StreamWhatYouHear/SWYH/releases",
    download_headers: &[],
    get_sig_key: None,
    fetch_update: |_, prev_version, response| {
      let release: github::Release = response.into_json().map_err(Error::UpdateIo)?;
      let version = release.tag.parse().map_err(|_| Error::UpdateParseVersion)?;
      if version > prev_version {
        let [url32] = github::extract_assets(release.assets, &["x32"], |x: &str| {
          x.starts_with("SWYH_") && x.ends_with(".exe")
        })?;
        Ok(Some(PackageUpdate {
          version,
          x32: Some(PlatformUpdate::new(url32)),
          x64: None,
        }))
      } else {
        Ok(None)
      }
    },
  },
  Package {
    name: "xysubfilter",
    version_url: "https://api.github.com/repos/Cyberbeing/xy-VSFilter/releases",
    download_headers: &[],
    get_sig_key: None,
    fetch_update: |_, prev_version, response| {
      let release: github::Release = response.into_json().map_err(Error::UpdateIo)?;
      let version = release.tag.parse().map_err(|_| Error::UpdateParseVersion)?;
      if version > prev_version {
        let [url32, url64] = github::extract_assets(
          release.assets,
          &["x86", "x64"],
          (
            |x: &str| x.ends_with("_x86.zip"),
            |x: &str| x.ends_with("_x64.zip"),
          ),
        )?;
        Ok(Some(PackageUpdate {
          version,
          x32: Some(PlatformUpdate::new(url32)),
          x64: Some(PlatformUpdate::new(url64)),
        }))
      } else {
        Ok(None)
      }
    },
  },
];

fn main() {
  const THREAD_COUNT: usize = 4;

  let api_key = env::args().nth(1).expect("missing api key");

  let agent = ureq::AgentBuilder::new()
    .max_idle_connections(4)
    .timeout(Duration::from_secs(10))
    .build();

  let mut up_to_date = Vec::with_capacity(PACKAGES.len());
  let mut updated = Vec::with_capacity(PACKAGES.len());
  let mut errors = Vec::with_capacity(PACKAGES.len());
  let mut report = String::new();

  thread::scope(|scope| {
    let (package_send, package_recv) = crossbeam_channel::bounded(PACKAGES.len());
    let (result_send, result_recv) = crossbeam_channel::bounded(PACKAGES.len());

    for p in PACKAGES {
      package_send.send(p).unwrap();
    }
    drop(package_send);

    for _ in 0..THREAD_COUNT {
      let result_send = result_send.clone();
      let package_recv = package_recv.clone();
      let agent = &agent;

      scope.spawn(move || {
        for package in package_recv {
          let res = package.process(agent);
          result_send.send((package.name, res)).unwrap();
        }
      });
    }
    drop(result_send);

    for (package, result) in result_recv {
      match result {
        Ok(UpdateStatus::UpToDate(version)) => up_to_date.push((package, version)),
        Ok(UpdateStatus::Updated { prev, new }) => {
          if let Err(e) = command::git_commit_package(package, new) {
            errors.push((package, e))
          } else {
            updated.push((package, prev, new))
          }
        }
        Err(e) => errors.push((package, e)),
      }
    }
  });

  // Make sure changes are pushed to git first.
  if !updated.is_empty() && command::git_push().is_err() {
    errors.extend(updated.drain(..).map(|(package, ..)| (package, Error::PushFailed)));
  }

  // Attempt to push everything to chocolatey.
  let reset_start = errors.len();
  for i in (0..updated.len()).rev() {
    let update = &updated[i];
    if let Err(e) = command::choco_push_package(update.0, update.2, &api_key) {
      errors.push((update.0, e));
      updated.remove(i);
    }
  }

  // Attempt to roll back any packages which couldn't be pushed to chocolatey.
  if reset_start != errors.len()
    && (command::git_reset(updated.len() + (errors.len() - reset_start)).is_err()
      || updated
        .iter()
        .any(|&(package, _, new)| command::git_commit_package(package, new).is_err())
      || command::git_force_push().is_err())
  {
    let _ = write!(report, "The following packages have been updated in the repo, but could not be pushed to chocolatey:\n\n");
    for &(package, _) in &errors[reset_start..] {
      let _ = writeln!(report, "- {package}");
    }
    let _ = writeln!(report);
  }

  errors.sort_by_key(|x| x.0);
  updated.sort_by_key(|x| x.0);
  up_to_date.sort_by_key(|x| x.0);

  if !errors.is_empty() {
    let _ = write!(report, "## Errors\n\n");
    for (package, error) in &errors {
      let _ = writeln!(report, "- {package}: {error}");
    }
    if !updated.is_empty() && !up_to_date.is_empty() {
      let _ = writeln!(report,);
    }
  }
  if !updated.is_empty() {
    let _ = write!(report, "## Updates\n\n");
    for (package, prev, new) in &updated {
      let _ = writeln!(report, "- {package}: {prev} -> {new}");
    }
    if !up_to_date.is_empty() {
      let _ = writeln!(report,);
    }
  }
  if !up_to_date.is_empty() {
    let _ = write!(report, "## Up to date\n\n");
    for (package, version) in &up_to_date {
      let _ = writeln!(report, "- {package}: {version}");
    }
  }

  if !env::var("GITHUB_STEP_SUMMARY").is_ok_and(|file| fs::write(&file, &report).is_ok()) {
    println!("{report}")
  }

  if !errors.is_empty() {
    exit(1);
  }
}
