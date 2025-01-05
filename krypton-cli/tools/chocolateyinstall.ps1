$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
$url32 = 'https://github.com/zobo/kr/releases/download/kr-2.4.15-win3/kr-2.4.15-win3.zip'
$checksum32 = '7c855c1e36bc2d54674511e069c881f1f170f86b6ebc767afed484a9cbe93fa3'

$packageArgs = @{
  packageName   = 'krypton-cli'
  unzipLocation = $toolsDir
  fileType      = 'zip'
  url           = $url32
  checksum      = $checksum32
  checksumType  = 'sha256'
}

Install-ChocolateyZipPackage @packageArgs
