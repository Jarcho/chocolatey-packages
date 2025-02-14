$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.44.6/nextdns_1.44.6_windows_386.zip'
$checksum32 = '5060818bbc66cac3d5d9b835431ec9aa348b6b98e8db1b48ed8088edf42c7a1d'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.44.6/nextdns_1.44.6_windows_amd64.zip'
$checksum64 = '71aad4bdac9b17b9fb62af73c7c92cbe1684b5d7dd5991268d0939ea30703b6e'

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
