$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$installDir32   = "$(Join-Path $toolsDir x86)"
$url32      = 'https://github.com/Blitzker/assfiltermod/releases/download/0.4/assfiltermod_v0.4_x32.zip'
$checksum32 = '138003e042fdf7050ed192c5087de42cabb816371fa27db40ea599131e1a96f0'
$url64      = 'https://github.com/Blitzker/assfiltermod/releases/download/0.4/assfiltermod_v0.4_x64.zip'
$checksum64 = '77074ed9c28d12ed07569cbe293a6765ab7f2696034edba97875cfc92d5eb5ac'

$packageArgs = @{
  packageName    = 'assfiltermod'
  unzipLocation  = $toolsDir
  fileType       = 'zip'
  url            = $url32
  checksumType   = 'sha256'
  checksum       = $checksum32
  url64bit       = $url64
  checksumType64   = 'sha256'
  checksum64       = $checksum64
}

$packageArgs32 = @{
  packageName   = 'assfiltermod'
  unzipLocation = $installDir32
  fileType      = 'zip'
  url           = $url32
  checksumType  = 'sha256'
  checksum      = $checksum32
}

Install-ChocolateyZipPackage @packageArgs
regsvr32.exe "$(Join-Path $toolsDir assfiltermod.dll)" /s

if (Get-OSArchitectureWidth -eq 64) {
  Install-ChocolateyZipPackage @packageArgs32
  regsvr32.exe "$(Join-Path $installDir32 assfiltermod.dll)" /s
}
