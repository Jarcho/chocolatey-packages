$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/WerWolv/ImHex/releases/download/v1.38.0/imhex-1.38.0-Windows-x86_64.msi'
$checksum64 = 'c80aab1ce26cb98afd1eab3bbea743fcc537caf186640a08e79bb5d701bcecd3'

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
