$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/jj-vcs/jj/releases/download/v0.40.0/jj-v0.40.0-x86_64-pc-windows-msvc.zip'
$checksum64 = '63922bd257f9616553dec0869e2de99c1c0bf8d951c774d230af09eaeb2f5951'

$packageArgs = @{
  packageName    = 'jujutsu.portable'
  unzipLocation  = $toolsDir
  fileType       = 'zip'
  url64            = $url64
  checksumType64   = 'sha256'
  checksum64       = $checksum64
}

Install-ChocolateyZipPackage @packageArgs
