$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/jj-vcs/jj/releases/download/v0.45.1/jj-v0.45.1-x86_64-pc-windows-msvc.zip'
$checksum64 = '5dbf2619272c897394d34190a94b21a6ab8d1e8e59fcf7dfc173d8fa8aa90bf0'

$packageArgs = @{
  packageName    = 'jujutsu.portable'
  unzipLocation  = $toolsDir
  fileType       = 'zip'
  url64            = $url64
  checksumType64   = 'sha256'
  checksum64       = $checksum64
}

Install-ChocolateyZipPackage @packageArgs
