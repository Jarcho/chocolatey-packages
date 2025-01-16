use crate::nuspec::Version;
use crate::{Error, Result};
use std::os::windows::process::CommandExt;
use std::process::Command;

fn run_command(name: &'static str, command: &mut Command) -> Result<()> {
  let status = command.status().map_err(|e| Error::CommandIo(name, e))?;
  if status.success() {
    Ok(())
  } else {
    Err(Error::CommandStatus(name, status))
  }
}

pub fn git_commit_package(package: &str, version: Version) -> Result<()> {
  run_command(
    "git commit",
    Command::new("git").raw_arg(format!(
      "commit \
        --author=\"Github Actions <jarcho@users.noreply.github.com>\" \
        --message=\"Updated `{package}` to `{version}`\" \
        \"{package}/{package}.nuspec\" \
        \"{package}/tools/chocolateyinstall.ps1\""
    )),
  )
}

pub fn git_push() -> Result<()> {
  run_command("git push", Command::new("git").arg("push"))
}

pub fn git_force_push() -> Result<()> {
  run_command(
    "git push",
    Command::new("git").args(["push", "--force-with-lease"]),
  )
}

pub fn git_reset(count: usize) -> Result<()> {
  run_command(
    "git reset",
    Command::new("git").args(["reset", format!("HEAD~{count}").as_str()]),
  )
}

pub fn choco_push_package(package: &str, version: Version, api_key: &str) -> Result<()> {
  run_command(
    "choco pack",
    Command::new("choco").args(["pack", &format!(".\\{package}\\{package}.nuspec")]),
  )?;
  run_command(
    "choco push",
    Command::new("choco").args([
      "push",
      format!(".\\{package}\\{package}.{version}.nupkg").as_str(),
      "--source",
      "'https://push.chocolatey.org/'",
      "--api-key",
      api_key,
    ]),
  )
}
