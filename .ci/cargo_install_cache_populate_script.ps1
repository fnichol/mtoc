Param(
    [Switch]
    $help,

    [Parameter(Mandatory=$True)]
    [String]
    $plugin
)

function Write-Help {
    Write-Host @"
$program

Prints latest version of a Cargo crate

USAGE:
    $program [FLAGS] <CRATE>

FLAGS:
    -help Prints this message

"@
}

function main() {
    $script:program = "cargo_install_cache_populate_script"

    if ($help) {
        Write-Help
        exit
    }

    $root = "$env:CARGO_HOME\opt\$plugin"

    Install-Plugin "$plugin" "$root"
}

function Install-Plugin {
    [CmdletBinding()]
    Param(
        [Parameter(Mandatory=$True)]
        [String]
        $plugin,

        [Parameter(Mandatory=$True)]
        [String]
        $root
    )

    if (-Not (Test-Path "$root")) {
        New-Item -Type Directory "$root" | Out-Null
    }
    cargo install --root "$root" --force --verbose "$plugin"

    # Create symbolic links for all execuatbles into $env:CARGO_HOME\bin
    Get-ChildItem "$root\bin\*.exe" | ForEach-Object {
        $dst = "$env:CARGO_HOME\bin\$($_.Name)"

        if (-Not (Test-Path "$dst")) {
            New-Item -Path "$dst" -Type SymbolicLink -Value "$_" | Out-Null
        }
    }
}

main
