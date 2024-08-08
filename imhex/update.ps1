Import-Module au

function global:au_GetLatest {
    $response = Invoke-WebRequest "https://api.github.com/repos/WerWolv/ImHex/releases"
    $releases = $response | ConvertFrom-Json
    $latest_release = $releases | Where-Object {$_.prerelease -eq $false} | select -First 1

    $version = $latest_release.tag_name.TrimStart("v")

    foreach ($asset in $latest_release.assets) {
        $windows_asset = $asset | ? name -Match 'Windows(-x86_64)?\.zip'
        if ($windows_asset) {
            $download_url = $windows_asset.browser_download_url
            break
        }
    }

    return @{
        Version = $version
        URL64   = $download_url
    }
}

function global:au_SearchReplace {
    @{
        "tools\chocolateyInstall.ps1" = @{
            "(?i)(^\`$url64\s*=\s*)('.*')"      = "`$1'$($Latest.URL64)'"
            "(?i)(^\`$checksum64\s*=\s*)('.*')" = "`$1'$($Latest.Checksum64)'"
        }
    }
}

update
