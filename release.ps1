# Release automation script for CommentsCleaner (PowerShell version)
# This script helps prepare releases and can be used for local testing

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    
    [switch]$DryRun,
    [switch]$BuildOnly,
    [switch]$PublishCargo,
    [switch]$Help
)

# Colors for output
$Colors = @{
    Red = "Red"
    Green = "Green"
    Yellow = "Yellow"
    Blue = "Blue"
    White = "White"
}

function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Colors[$Color]
}

function Write-Info {
    param([string]$Message)
    Write-ColorOutput "[INFO] $Message" "Blue"
}

function Write-Success {
    param([string]$Message)
    Write-ColorOutput "[SUCCESS] $Message" "Green"
}

function Write-Warning {
    param([string]$Message)
    Write-ColorOutput "[WARNING] $Message" "Yellow"
}

function Write-Error {
    param([string]$Message)
    Write-ColorOutput "[ERROR] $Message" "Red"
}

function Show-Usage {
    @"
Usage: .\release.ps1 -Version VERSION [OPTIONS]

REQUIRED:
    -Version VERSION     Set release version (e.g., 0.1.0)

OPTIONS:
    -DryRun              Show what would be done without executing
    -BuildOnly           Only build binaries, don't create release
    -PublishCargo        Publish to crates.io (requires CRATES_IO_TOKEN)
    -Help                Show this help message

EXAMPLES:
    .\release.ps1 -Version 0.1.0                    # Build and prepare release v0.1.0
    .\release.ps1 -Version 0.1.0 -DryRun            # Show what would be done
    .\release.ps1 -Version 0.1.0 -BuildOnly         # Only build binaries
    .\release.ps1 -Version 0.1.0 -PublishCargo      # Build and publish to crates.io

"@
}

if ($Help) {
    Show-Usage
    exit 0
}

# Validate version format
if ($Version -notmatch '^\d+\.\d+\.\d+$') {
    Write-Error "Version must be in format x.y.z (e.g., 0.1.0)"
    exit 1
}

Write-Info "Preparing release for version $Version"

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Error "Cargo.toml not found. Please run this script from the project root."
    exit 1
}

# Update version in Cargo.toml
Write-Info "Updating version in Cargo.toml to $Version"
if (-not $DryRun) {
    (Get-Content Cargo.toml) -replace '^version = .*', "version = `"$Version`"" | Set-Content Cargo.toml
}

# Targets to build
$Targets = @(
    @{ Target = "x86_64-unknown-linux-gnu"; Output = "cclean-linux-x86_64" }
    @{ Target = "x86_64-pc-windows-gnu"; Output = "cclean-windows-x86_64.exe" }
    @{ Target = "x86_64-apple-darwin"; Output = "cclean-macos-x86_64" }
)

# Create release directory
$ReleaseDir = "release-$Version"
Write-Info "Creating release directory: $ReleaseDir"
if (-not $DryRun) {
    New-Item -ItemType Directory -Force -Path $ReleaseDir | Out-Null
}

# Build for each target
foreach ($TargetInfo in $Targets) {
    $Target = $TargetInfo.Target
    $OutputName = $TargetInfo.Output
    
    Write-Info "Building for target: $Target"
    
    if (-not $DryRun) {
        # Install target if not already installed
        & rustup target add $Target
        
        # Build
        & cargo build --release --target $Target
        
        # Copy binary to release directory
        $BinaryPath = "target\$Target\release\cclean"
        if ($Target -like "*windows*") {
            $BinaryPath = "target\$Target\release\cclean.exe"
        }
        
        if (Test-Path $BinaryPath) {
            Copy-Item $BinaryPath "$ReleaseDir\$OutputName"
            
            # Create tar.gz
            Push-Location $ReleaseDir
            # Use tar command (available in Windows 10+)
            & tar -czf "${OutputName}.tar.gz" $OutputName
            Remove-Item $OutputName
            Pop-Location
            
            Write-Success "Built $OutputName"
        } else {
            Write-Error "Binary not found at $BinaryPath"
            exit 1
        }
    } else {
        Write-Info "[DRY RUN] Would build for $Target"
    }
}

# Create checksums
Write-Info "Creating checksums"
if (-not $DryRun) {
    Push-Location $ReleaseDir
    # Calculate SHA256 for each file
    $Files = Get-ChildItem -Filter "*.tar.gz"
    $ChecksumContent = @()
    foreach ($File in $Files) {
        $Hash = Get-FileHash $File -Algorithm SHA256
        $ChecksumContent += "$($Hash.Hash.ToLower())  $($File.Name)"
    }
    $ChecksumContent | Out-File -FilePath "sha256sums.txt" -Encoding UTF8
    Pop-Location
    Write-Success "Created sha256sums.txt"
}

# Show release info
Write-Info "Release files prepared:"
if ($DryRun) {
    Write-Info "[DRY RUN] Files would be in: $ReleaseDir/"
} else {
    Get-ChildItem $ReleaseDir | Format-Table Name, Length -AutoSize
}

# Git operations
if (-not $BuildOnly) {
    Write-Info "Preparing git operations..."
    
    if (-not $DryRun) {
        # Create git tag
        Write-Info "Creating git tag v$Version"
        & git add Cargo.toml
        & git commit -m "Bump version to $Version" 2>$null
        & git tag -a "v$Version" -m "Release version $Version"
        
        Write-Success "Created git tag v$Version"
        Write-Info "To push the tag: git push origin v$Version"
        Write-Info "To push commits: git push origin main"
    } else {
        Write-Info "[DRY RUN] Would create git tag v$Version"
    }
}

# Cargo publish
if ($PublishCargo) {
    Write-Info "Publishing to crates.io..."
    if (-not $DryRun) {
        & cargo publish
        Write-Success "Published to crates.io"
    } else {
        Write-Info "[DRY RUN] Would publish to crates.io"
    }
}

Write-Success "Release preparation complete!"
Write-Info "Next steps:"
if (-not $BuildOnly -and -not $DryRun) {
    @"
1. Review the changes: git status
2. Push commits: git push origin main
3. Push tag: git push origin v$Version
4. GitHub Actions will automatically build and create release
"@
} else {
    "1. Run without -DryRun to execute the release"
}

if ($BuildOnly) {
    "2. Upload the files in $ReleaseDir/ manually to GitHub release"
}
