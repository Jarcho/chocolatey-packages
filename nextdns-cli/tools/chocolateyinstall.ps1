$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.43.3/nextdns_1.43.3_windows_386.zip'
$checksum32 = '953058dc4099f44838923d46ab402319d8d2530756e78623a4130d082acde705'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.43.3/nextdns_1.43.3_windows_amd64.zip'
$checksum64 = 'bb729b9170c519041f52725a9b8f033f3dfaf3d62ec7782fc9adc7b23b7eb4c9'

$packageArgs = @{
  packageName    = 'nextdns-cli'
  unzipLocation  = $toolsDir
  fileType       = 'zip'
  url            = $url32
  checksumType   = 'sha256'
  checksum       = $checksum32
  url64bit       = $url64
  checksumType64   = 'sha256'
  checksum64       = $checksum64
}

Install-ChocolateyZipPackage @packageArgs
