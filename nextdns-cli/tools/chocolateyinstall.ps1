$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.44.5/nextdns_1.44.5_windows_386.zip'
$checksum32 = '5048254f9ecd83bc94451264daa2fb161c183ea256d76ad877d75d19bbb2bec3'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.44.5/nextdns_1.44.5_windows_amd64.zip'
$checksum64 = 'ac99a10d4c8ad48d9c1999313214118c5751ff4132814228a82c5dc0ba330750'

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
