$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.44.3/nextdns_1.44.3_windows_386.zip'
$checksum32 = '26c57de900a3e7e7dae79e9746cad2570607137ad713788789ef93bd0b1623c3'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.44.3/nextdns_1.44.3_windows_amd64.zip'
$checksum64 = '59d5a64dbac2c8c340676caaaa52d8d9d49da38e663df20f2b1b5860479f4ab7'

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
