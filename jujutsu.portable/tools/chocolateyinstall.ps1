$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/jj-vcs/jj/releases/download/v0.37.0/jj-v0.37.0-x86_64-pc-windows-msvc.zip'
$checksum64 = '8400a4021a94dbb29b292b776812ef0d17c125bc72603cfdf8e18d2f382bccb1'

$packageArgs = @{
  packageName    = 'jujutsu.portable'
  unzipLocation  = $toolsDir
  fileType       = 'zip'
  url64            = $url64
  checksumType64   = 'sha256'
  checksum64       = $checksum64
}

Install-ChocolateyZipPackage @packageArgs
