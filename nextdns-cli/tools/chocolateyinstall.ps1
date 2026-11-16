$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.50.2/nextdns_1.50.2_windows_386.zip'
$checksum32 = 'be7aaadea76aebce0b77ed066e36b88c8d0ceb769e45f35ce8802c67cffe99b6'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.50.2/nextdns_1.50.2_windows_amd64.zip'
$checksum64 = 'd61256a76783489a63e6e575cd1a34776878f8b4c2a601178dc09a6e49aea490'

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
