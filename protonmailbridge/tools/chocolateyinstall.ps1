$ErrorActionPreference = 'Stop';

$url64      = 'https://github.com/ProtonMail/proton-bridge/releases/download/v3.16.0/Bridge-Installer.exe'
$checksum64 = '224f234dab22cf3e3770363a67645396d74df701bfe3850ca1b4ddd4d8b4e50d'

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
