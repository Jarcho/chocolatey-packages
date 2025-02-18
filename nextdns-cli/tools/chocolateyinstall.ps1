$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.45.0/nextdns_1.45.0_windows_386.zip'
$checksum32 = '3fc03ff741f53b2067de90e17d12fb7413085c79e78eb98ac95f316c50edaea0'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.45.0/nextdns_1.45.0_windows_amd64.zip'
$checksum64 = '0fac2e82024c60a0a8a970b8575dc4fb4094204022ae53e8bdfbcfe1fbd17a86'

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
