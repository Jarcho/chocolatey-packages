$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.46.0/nextdns_1.46.0_windows_386.zip'
$checksum32 = '04633cb0e8af86969e1d19b1b5a6ea1b58bccebe9d2f898afc00088c74fcbede'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.46.0/nextdns_1.46.0_windows_amd64.zip'
$checksum64 = '5fbb39f6f88dac77d7227475f185b212d4b92b13c3b025954950e57c413c60f3'

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
