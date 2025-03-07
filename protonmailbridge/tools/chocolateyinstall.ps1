$ErrorActionPreference = 'Stop';

$url64      = 'https://github.com/ProtonMail/proton-bridge/releases/download/v3.18.0/Bridge-Installer.exe'
$checksum64 = '6de53128336a0f58c83395df71ae5c73cab0909c2242702e4c6436cdbaf76057'

$packageArgs = @{
    packageName    = 'protonmailbridge'
    fileType       = 'exe'
    silentArgs     = '/quiet'
    validExitCodes = @(0)
    url64          = $url64
    checksum64     = $checksum64
    checksumType   = 'sha256'
}

Install-ChocolateyPackage @packageArgs
