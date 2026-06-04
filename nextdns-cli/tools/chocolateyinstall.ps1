$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.47.3/nextdns_1.47.3_windows_386.zip'
$checksum32 = '652ed317f76acc8168ff2637c848abc9b0d5c76ee5c9115f20c4edbc81409c14'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.47.3/nextdns_1.47.3_windows_amd64.zip'
$checksum64 = 'b9ea2c3b2286d62aff63c48356c06ee75bf9fb775b7d9800b3c0d9febaedcbfc'

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
