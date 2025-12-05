$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/jj-vcs/jj/releases/download/v0.36.0/jj-v0.36.0-x86_64-pc-windows-msvc.zip'
$checksum64 = 'f173e51b54617b91de2a61f6e83a43220a91ec5025fa5d002a07668780074f61'

$packageArgs = @{
  packageName    = 'jujutsu.portable'
  unzipLocation  = $toolsDir
  fileType       = 'zip'
  url64            = $url64
  checksumType64   = 'sha256'
  checksum64       = $checksum64
}

Install-ChocolateyZipPackage @packageArgs
