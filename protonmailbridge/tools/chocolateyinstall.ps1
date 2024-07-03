$ErrorActionPreference = 'Stop';

$url32 = 'https://proton.me/download/bridge/Bridge-Installer.exe'
$checksum32 = '55b052a88d8b23c0241568da6de00283f282a46f3b820469b76f5188413cbe25'

$packageArgs = @{
    packageName    = 'protonmailbridge'
    fileType       = 'exe'
    silentArgs     = '/quiet'
    validExitCodes = @(0)
    url            = $url32
    checksum       = $checksum32
    checksumType   = 'sha256'
}

Install-ChocolateyPackage @packageArgs
