$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.43.4/nextdns_1.43.4_windows_386.zip'
$checksum32 = '3038a450b1ce2e3ccf6aa4c4c5ebf0eeb59c52aef4f73197df42bbe855a49c08'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.43.4/nextdns_1.43.4_windows_amd64.zip'
$checksum64 = '317459ad30bcc6bfe19e05b51d8f29a4af69f06c61c2df6cb2913a9caee02183'

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
