$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/WerWolv/ImHex/releases/download/v1.38.1/imhex-1.38.1-Windows-x86_64.msi'
$checksum64 = 'f0be16446f546ade94e55c3626a67c831cc85c03965a9c12bacf0c076214851c'

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
