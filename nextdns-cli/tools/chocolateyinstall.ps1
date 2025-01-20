$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.44.4/nextdns_1.44.4_windows_386.zip'
$checksum32 = '7aff950b9fc1386c10ecb61be0b416106c883bc1b4844cb9b0eca791e2266d2d'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.44.4/nextdns_1.44.4_windows_amd64.zip'
$checksum64 = '7ae96234be5b1cc934559aba86c6cba327bd714cfed8101fb19ab1621bc49f73'

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
