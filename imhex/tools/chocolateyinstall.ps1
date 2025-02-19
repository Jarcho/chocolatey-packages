$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/WerWolv/ImHex/releases/download/v1.37.1/imhex-1.37.1-Windows-x86_64.msi'
$checksum64 = 'c312ebc1313a1edd335c8f39094936f86e8a65f2711873b7f55297a46ca4a2cf'

$installArgs = @{
    packageName    = 'imhex'
    fileType       = 'msi'
    softwareName   = 'ImHex'
    url64          = $url64
    checksum64     = $checksum64
    checksumType   = 'sha256'
    silentArgs     = '/quiet /norestart ALLUSERS=1'
    validExitCodes = @(0, 3010, 1641)
}

Install-ChocolateyPackage @installArgs

$installDir = $toolsDir
New-Item "$(Join-Path $installDir 'ImHex.exe.gui')" -type file -Force | Out-Null
