$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/WerWolv/ImHex/releases/download/v1.37.0/imhex-1.37.0-Windows-x86_64.msi'
$checksum64 = 'a5c7a019ba65bdf58f9f2bacf46657dd7fa0f9eef7eb62edb5e7068251869a49'

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
