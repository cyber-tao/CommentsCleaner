# Release Guide

This document explains how to create releases for the CommentsCleaner project.

## Automated Releases

### GitHub Actions Workflows

We have two automated release workflows:

#### 1. Tag-based Release (`release.yml`)

Triggered automatically when you push a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This will:
- Create a GitHub release
- Build binaries for Linux, Windows, and macOS
- Upload binaries to the release
- Optionally publish to crates.io

#### 2. Manual Release (`build-release.yml`)

Can be triggered manually from GitHub Actions tab:
- Go to Actions → Build and Release → "Run workflow"
- Specify version and options
- Choose whether to create GitHub release and/or publish to crates.io

## Local Release Scripts

### Linux/macOS (`release.sh`)

```bash
# Build and prepare release
./release.sh -v 0.1.0

# Dry run (show what would be done)
./release.sh -v 0.1.0 --dry-run

# Build only (no git operations)
./release.sh -v 0.1.0 --build-only

# Build and publish to crates.io
./release.sh -v 0.1.0 --publish-cargo
```

### Windows (`release.ps1`)

```powershell
# Build and prepare release
.\release.ps1 -Version 0.1.0

# Dry run
.\release.ps1 -Version 0.1.0 -DryRun

# Build only
.\release.ps1 -Version 0.1.0 -BuildOnly

# Build and publish to crates.io
.\release.ps1 -Version 0.1.0 -PublishCargo
```

## Release Process

### 1. Preparation

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release notes
3. Test the build locally
4. Ensure all tests pass

### 2. Create Release

#### Option A: Automated (Recommended)

```bash
# Create and push tag
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

GitHub Actions will handle the rest.

#### Option B: Manual with Scripts

```bash
# Use local script
./release.sh -v 0.1.0

# Push changes
git push origin main
git push origin v0.1.0
```

### 3. Post-Release

1. Verify the GitHub release
2. Test downloaded binaries
3. Update documentation if needed
4. Announce the release

## Configuration

### `release.toml`

Configure release behavior in `release.toml`:

```toml
[release]
targets = ["x86_64-unknown-linux-gnu", "x86_64-pc-windows-gnu", "x86_64-apple-darwin"]

[settings]
auto_create_release = true
auto_publish_cargo = false
strip_binaries = true
create_checksums = true
```

### Required Secrets

For automated releases, configure these secrets in GitHub repository settings:

- `GITHUB_TOKEN`: Automatically provided by GitHub Actions
- `CRATES_IO_TOKEN`: Optional, for publishing to crates.io

## Build Targets

Currently supported targets:

| Target | Platform | Binary Name |
|--------|----------|-------------|
| `x86_64-unknown-linux-gnu` | Linux x64 | `cclean-linux-x86_64` |
| `x86_64-pc-windows-gnu` | Windows x64 | `cclean-windows-x86_64.exe` |
| `x86_64-apple-darwin` | macOS x64 | `cclean-macos-x86_64` |

## Release Artifacts

Each release includes:

- Compressed binary for each platform
- `sha256sums.txt` for verification
- Release notes with installation instructions

## Troubleshooting

### Build Failures

1. Check Rust toolchain version
2. Verify target installation: `rustup target add <target>`
3. Check system dependencies

### Release Issues

1. Ensure tag format: `vx.y.z`
2. Check GitHub Actions permissions
3. Verify secrets configuration

### Cargo Publish Issues

1. Check `CRATES_IO_TOKEN` is valid
2. Ensure version number is incremented
3. Verify `Cargo.toml` is valid

## Versioning

We follow Semantic Versioning:

- `MAJOR.MINOR.PATCH`
- Major: Breaking changes
- Minor: New features (backward compatible)
- Patch: Bug fixes (backward compatible)

Example: `v0.1.0`, `v0.1.1`, `v0.2.0`

## Testing Before Release

Always test before releasing:

```bash
# Run full test suite
cargo test

# Build for all targets
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin

# Test release script (dry run)
./release.sh -v 0.1.0 --dry-run
```
