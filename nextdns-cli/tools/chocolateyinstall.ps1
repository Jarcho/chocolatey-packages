$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.49.0/nextdns_1.49.0_windows_386.zip'
$checksum32 = '313aa5f17e3fd27fdd4cf1242d817c5cc43c911994914c9d88bc4c57e4d0d693'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.49.0/nextdns_1.49.0_windows_amd64.zip'
$checksum64 = '1218e6aad99787b1489f5d9b26c5af163ec26d5331c0c78a44faf0bcd88c1b7f'

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
