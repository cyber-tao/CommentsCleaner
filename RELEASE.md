# Release Guide

This document explains how to create releases for the CommentsCleaner project.

## 🚀 Automated Release Process

Releases are **fully automated** through GitHub Actions. Simply push a version tag to trigger the release workflow.

### Quick Start

```bash
# 1. Update version in Cargo.toml
# 2. Commit changes
git add Cargo.toml
git commit -m "Bump version to 0.2.0"

# 3. Create and push tag
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

**That's it!** GitHub Actions will automatically:
- ✅ Build Linux x86_64 binary
- ✅ Build Windows x86_64 binary (GNU toolchain)
- ✅ Create GitHub Release with version tag
- ✅ Upload binaries with versioned filenames
- ✅ (Optional) Publish to crates.io if configured

### What Gets Built

| Platform | Target | Output |
|----------|--------|--------|
| Linux | `x86_64-unknown-linux-gnu` | `cclean-{version}-linux-x86_64` |
| Windows | `x86_64-pc-windows-gnu` | `cclean-{version}-windows-x86_64.exe` |

Example for v0.1.0:
- `cclean-v0.1.0-linux-x86_64`
- `cclean-v0.1.0-windows-x86_64.exe`

## 📋 Detailed Release Steps

### 1. Preparation

Before creating a release:

1. **Update version** in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"
   ```

2. **Update CHANGELOG.md** (if exists) with:
   - New features
   - Bug fixes
   - Breaking changes
   - Notable improvements

3. **Run tests locally**:
   ```bash
   cargo test --all-features
   cargo clippy --all-targets --all-features
   cargo fmt --check
   ```

4. **Build and test locally**:
   ```bash
   cargo build --release
   ./target/release/cclean --version
   ```

### 2. Create Release

#### Commit version bump:
```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Release version 0.2.0"
git push origin main
```

#### Create and push tag:
```bash
# Create annotated tag
git tag -a v0.2.0 -m "Release version 0.2.0"

# Push tag to trigger release workflow
git push origin v0.2.0
```

### 3. Monitor Release

1. Go to **GitHub Actions** tab
2. Watch the **Release** workflow
3. Two parallel jobs will run:
   - `release (x86_64-unknown-linux-gnu)`
   - `release (x86_64-pc-windows-gnu)`
4. Wait for completion (~2-3 minutes)

### 4. Verify Release

1. Go to **Releases** page on GitHub
2. Check that `v0.2.0` release exists
3. Verify both binaries are uploaded:
   - `cclean-v0.2.0-linux-x86_64`
   - `cclean-v0.2.0-windows-x86_64.exe`
4. Test download links work

## 🔧 Configuration

### GitHub Actions Workflow

The release workflow is defined in `.github/workflows/release.yml`:

- **Trigger**: Push tags matching `v*` pattern
- **Platforms**: Linux (Ubuntu), cross-compiles Windows
- **Build**: Rust stable toolchain
- **Upload**: softprops/action-gh-release@v1

### Required Secrets

Configured in GitHub Repository Settings → Secrets:

- `GITHUB_TOKEN`: **Automatically provided** by GitHub Actions (no setup needed)
- `CRATES_IO_TOKEN`: Optional, only needed if publishing to crates.io

### Optional: Publish to crates.io

To enable automatic publishing to crates.io:

1. Generate token at https://crates.io/me
2. Add as `CRATES_IO_TOKEN` secret in GitHub
3. Workflow will automatically publish when tag is pushed

## 📦 Release Artifacts

Each release includes:

1. **Linux Binary**: Native ELF executable, stripped
2. **Windows Binary**: PE executable built with mingw-w64
3. **Release Notes**: Auto-generated from tag message
4. **Download URLs**: Direct links to each binary

## ❗ Troubleshooting

### Workflow Fails

**Check workflow logs:**
```bash
gh run list --workflow=release.yml --limit 5
gh run view <run-id> --log
```

**Common issues:**
- Missing Rust targets: Workflow installs automatically
- Permission denied: Check `contents: write` permission in workflow
- Upload fails: Verify `GITHUB_TOKEN` has proper permissions

### Binary Not Uploaded

**Symptoms:** Release created but no assets

**Solutions:**
1. Check workflow logs for "Create Release and Upload" step
2. Verify file exists in workspace before upload
3. Check file path matches in workflow

### Wrong Version Number in Filename

**Cause:** Tag name doesn't match expected format

**Solution:** Ensure tag follows format `vX.Y.Z` (e.g., `v0.1.0`)

## 🏷️ Versioning

We follow [Semantic Versioning](https://semver.org/):

```
vMAJOR.MINOR.PATCH
```

- **MAJOR**: Breaking changes (incompatible API)
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

Examples:
- `v0.1.0` - Initial release
- `v0.1.1` - Bug fix
- `v0.2.0` - New features
- `v1.0.0` - Stable release

## 🔄 Rolling Back a Release

If you need to remove a bad release:

```bash
# Delete release on GitHub
gh release delete v0.2.0 --yes

# Delete tag locally and remotely
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0

# Fix issues, then recreate tag
git tag v0.2.0
git push origin v0.2.0
```

## 📝 Best Practices

1. **Always test before release**: Run full test suite locally
2. **Use meaningful version numbers**: Follow semantic versioning
3. **Write clear release notes**: Document what changed
4. **Monitor workflow**: Watch the GitHub Actions run
5. **Verify artifacts**: Test downloaded binaries
6. **Tag annotated**: Use `git tag -a` for better Git history

## 🎯 Quick Reference

```bash
# Full release workflow
cargo test && cargo clippy
# Edit Cargo.toml version
git add Cargo.toml
git commit -m "Bump version to X.Y.Z"
git push origin main
git tag -a vX.Y.Z -m "Release version X.Y.Z"
git push origin vX.Y.Z

# View releases
gh release list

# View specific release
gh release view vX.Y.Z

# Download release assets
gh release download vX.Y.Z

# Monitor workflow
gh run watch
```

## 🆘 Getting Help

- **Workflow issues**: Check `.github/workflows/release.yml`
- **Build errors**: Check GitHub Actions logs
- **General questions**: Open an issue on GitHub

---

**Remember:** The entire release process is automated. Just push a tag and let GitHub Actions do the work!
