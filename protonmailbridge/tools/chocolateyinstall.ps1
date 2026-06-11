$ErrorActionPreference = 'Stop';

$url64      = 'https://github.com/ProtonMail/proton-bridge/releases/download/v3.25.0/Bridge-Installer.exe'
$checksum64 = '0cc9c525f4d0be1cd8c864bc1341ba2933f4057db6a3b4b191cfb0b6a6c24450'

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
