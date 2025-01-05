Import-Module au

function global:au_GetLatest {
    $url = 'https://nextdns.io/download/windows/stable'

    try {
        $response = Invoke-WebRequest -Uri "$url" -MaximumRedirection 0 -ErrorAction Ignore
    }
    catch {
        $response = $_.Exception.Response
    }

    if ($response.Headers.Location.gettype() -eq [string]) {
        $location = $response.Headers.Location
    }
    else {
        $location = $response.Headers.Location.AbsolutePath
    }

    $file = $location.SubString($location.LastIndexOf('/') + 1)
    $file -match 'NextDNSSetup-(?<version>[0-9.]*)\.exe'
    return @{ Version = $matches['version']; URL32 = $url }
}

function global:au_SearchReplace {
    @{
        "tools\chocolateyInstall.ps1" = @{
            "(?i)(^\`$url32\s*=\s*)('.*')"      = "`$1'$($Latest.URL)'"
            "(?i)(^\`$checksum32\s*=\s*)('.*')" = "`$1'$($Latest.Checksum)'"
        }
    }
}

update
