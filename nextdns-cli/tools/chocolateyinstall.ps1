$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url32      = 'https://github.com/nextdns/nextdns/releases/download/v1.43.0/nextdns_1.43.0_windows_386.zip'
$checksum32 = '7aa86ffcec2a20801594826b388adec54ccbe0e42d43aff551f823f38408dfa5'
$url64      = 'https://github.com/nextdns/nextdns/releases/download/v1.43.0/nextdns_1.43.0_windows_amd64.zip'
$checksum64 = 'ab4ed3476119bf55ddfbcacb3a4969e33b78639fa2e3896450014de2b18c9459'

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
