$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.43.5/nextdns_1.43.5_windows_386.zip'
$checksum32 = '7c7d513ea1d4b6a04c17878512717986e78cfa2e0632c6ed85981a0b02bceec0'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.43.5/nextdns_1.43.5_windows_amd64.zip'
$checksum64 = '257b898d1a2f9fd17ed875dd6207d8c840c97ea3fe1b880e995ceb93a9c41f4f'

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
