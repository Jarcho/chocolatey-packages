$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/jj-vcs/jj/releases/download/v0.30.0/jj-v0.30.0-x86_64-pc-windows-msvc.zip'
$checksum64 = '4b094838898bebd3970ef6c94b9cf5fef4368a0291e33ae884736195abce0671'

$packageArgs = @{
  packageName    = 'jujutsu.portable'
  unzipLocation  = $toolsDir
  fileType       = 'zip'
  url64            = $url64
  checksumType64   = 'sha256'
  checksum64       = $checksum64
}

Install-ChocolateyZipPackage @packageArgs
