#!/bin/bash

# Release automation script for CommentsCleaner
# This script helps prepare releases and can be used for local testing

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
VERSION=""
DRY_RUN=false
BUILD_ONLY=false
PUBLISH_CARGO=false

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

OPTIONS:
    -v, --version VERSION     Set release version (e.g., 0.1.0)
    -d, --dry-run             Show what would be done without executing
    -b, --build-only          Only build binaries, don't create release
    -c, --publish-cargo       Publish to crates.io (requires CRATES_IO_TOKEN)
    -h, --help                Show this help message

EXAMPLES:
    $0 -v 0.1.0                    # Build and prepare release v0.1.0
    $0 -v 0.1.0 --dry-run          # Show what would be done
    $0 -v 0.1.0 --build-only       # Only build binaries
    $0 -v 0.1.0 --publish-cargo    # Build and publish to crates.io

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -d|--dry-run)
            DRY_RUN=true
            shift
            ;;
        -b|--build-only)
            BUILD_ONLY=true
            shift
            ;;
        -c|--publish-cargo)
            PUBLISH_CARGO=true
            shift
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Validate version
if [[ -z "$VERSION" ]]; then
    print_error "Version is required. Use -v or --version to specify."
    show_usage
    exit 1
fi

# Validate version format
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "Version must be in format x.y.z (e.g., 0.1.0)"
    exit 1
fi

print_info "Preparing release for version $VERSION"

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    print_error "Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

# Update version in Cargo.toml
print_info "Updating version in Cargo.toml to $VERSION"
if [[ "$DRY_RUN" == "false" ]]; then
    sed -i.bak "s/^version = .*/version = \"$VERSION\"/" Cargo.toml
    rm Cargo.toml.bak
fi

# Targets to build
TARGETS=(
    "x86_64-unknown-linux-gnu:cclean-linux-x86_64"
    "x86_64-pc-windows-gnu:cclean-windows-x86_64.exe"
    "x86_64-apple-darwin:cclean-macos-x86_64"
)

# Create release directory
RELEASE_DIR="release-$VERSION"
print_info "Creating release directory: $RELEASE_DIR"
if [[ "$DRY_RUN" == "false" ]]; then
    mkdir -p "$RELEASE_DIR"
fi

# Build for each target
for target_info in "${TARGETS[@]}"; do
    IFS=':' read -r target output_name <<< "$target_info"
    
    print_info "Building for target: $target"
    
    if [[ "$DRY_RUN" == "false" ]]; then
        # Install target if not already installed
        rustup target add "$target"
        
        # Build
        cargo build --release --target "$target"
        
        # Copy binary to release directory
        BINARY_PATH="target/$target/release/cclean"
        if [[ "$target" == *"windows"* ]]; then
            BINARY_PATH="target/$target/release/cclean.exe"
        fi
        
        if [[ -f "$BINARY_PATH" ]]; then
            cp "$BINARY_PATH" "$RELEASE_DIR/$output_name"
            
            # Strip binary for non-Windows targets
            if [[ "$target" != *"windows"* ]]; then
                strip "$RELEASE_DIR/$output_name" 2>/dev/null || true
            fi
            
            # Create tar.gz
            cd "$RELEASE_DIR"
            tar -czf "${output_name}.tar.gz" "$output_name"
            rm "$output_name"
            cd ..
            
            print_success "Built $output_name"
        else
            print_error "Binary not found at $BINARY_PATH"
            exit 1
        fi
    else
        print_info "[DRY RUN] Would build for $target"
    fi
done

# Create checksums
print_info "Creating checksums"
if [[ "$DRY_RUN" == "false" ]]; then
    cd "$RELEASE_DIR"
    sha256sum *.tar.gz > "sha256sums.txt"
    cd ..
    print_success "Created sha256sums.txt"
fi

# Show release info
print_info "Release files prepared:"
if [[ "$DRY_RUN" == "true" ]]; then
    echo "[DRY RUN] Files would be in: $RELEASE_DIR/"
else
    ls -la "$RELEASE_DIR/"
fi

# Git operations
if [[ "$BUILD_ONLY" == "false" ]]; then
    print_info "Preparing git operations..."
    
    if [[ "$DRY_RUN" == "false" ]]; then
        # Create git tag
        print_info "Creating git tag v$VERSION"
        git add Cargo.toml
        git commit -m "Bump version to $VERSION" || true
        git tag -a "v$VERSION" -m "Release version $VERSION"
        
        print_success "Created git tag v$VERSION"
        print_info "To push the tag: git push origin v$VERSION"
        print_info "To push commits: git push origin main"
    else
        print_info "[DRY RUN] Would create git tag v$VERSION"
    fi
fi

# Cargo publish
if [[ "$PUBLISH_CARGO" == "true" ]]; then
    print_info "Publishing to crates.io..."
    if [[ "$DRY_RUN" == "false" ]]; then
        cargo publish
        print_success "Published to crates.io"
    else
        print_info "[DRY RUN] Would publish to crates.io"
    fi
fi

print_success "Release preparation complete!"
print_info "Next steps:"
if [[ "$BUILD_ONLY" == "false" && "$DRY_RUN" == "false" ]]; then
    echo "1. Review the changes: git status"
    echo "2. Push commits: git push origin main"
    echo "3. Push tag: git push origin v$VERSION"
    echo "4. GitHub Actions will automatically build and create release"
else
    echo "1. Run without --dry-run to execute the release"
fi

if [[ "$BUILD_ONLY" == "true" ]]; then
    echo "2. Upload the files in $RELEASE_DIR/ manually to GitHub release"
fi
