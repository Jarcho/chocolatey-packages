$ErrorActionPreference = 'Stop';

$url64      = 'https://github.com/ProtonMail/proton-bridge/releases/download/v3.27.2/Bridge-Installer.exe'
$checksum64 = '60be9e343840391629dd692c9b3339dee98ee6ff140e150dcb3866921c3e3268'

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
