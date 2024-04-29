$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.43.1/nextdns_1.43.1_windows_386.zip'
$checksum32 = 'eaa17906eadc9f3fc3128044fcdf43bf04bbd665f4750fe559eb003d20f53ef7'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.43.1/nextdns_1.43.1_windows_amd64.zip'
$checksum64 = 'cfd47c113ccf90ae63819817a13d59dd1bbcb33a91d848133d4881f5a48a31b6'

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
