$ErrorActionPreference = 'Stop';

$url64      = 'https://github.com/ProtonMail/proton-bridge/releases/download/v3.21.1/Bridge-Installer.exe'
$checksum64 = 'a31d97c945e9513b974c40b23bff892cff3e6a8b51ad6fa6961ebccfc6a2dc8e'

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
