$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.47.1/nextdns_1.47.1_windows_386.zip'
$checksum32 = '9507fa2bc5a9ae19fa1070c074b05dbbeca6b1a0b72b2f5b991d3699988589d2'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.47.1/nextdns_1.47.1_windows_amd64.zip'
$checksum64 = '9bb1dedc6263606d48cf479921146f89ea458328ee9629f9d79686bc7d1a3e36'

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
