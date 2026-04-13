$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.47.2/nextdns_1.47.2_windows_386.zip'
$checksum32 = '9fb0e3f2c660967352b532727a9e3589c8ebe6b445b1e32ea99d0b79e3296f96'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.47.2/nextdns_1.47.2_windows_amd64.zip'
$checksum64 = '516e1cd5a15602ca0dc1a4e45e2652d064df07bf6b15f1cf76cb7c1629d728c6'

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
