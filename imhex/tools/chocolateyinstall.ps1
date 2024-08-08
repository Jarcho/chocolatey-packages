$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
$url64 = 'https://github.com/WerWolv/ImHex/releases/download/v1.35.4/imhex-1.35.4-Windows-x86_64.msi'
$checksum64 = '11992c03b74f11c0f7be7770cad2cf96ba50b544588a1f2ca0222a600a807ab5'

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
