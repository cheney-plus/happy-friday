#!/bin/bash
set -euo pipefail

APP_ID="com.cheney.happy-friday"
MANIFEST="flatpak/${APP_ID}.yml"
BUILD_DIR=".flatpak-build"
REPO_DIR=".flatpak-repo"
RUNTIME_VERSION="46"
FDK_VERSION="24.08"
BUNDLE_NAME="${APP_ID}.flatpak"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

check_prerequisites() {
    info "Checking prerequisites..."

    if [[ "$(uname -s)" != "Linux" ]]; then
        error "Flatpak builds can only be performed on Linux. You are running $(uname -s). Consider using a Linux VM or container (Docker/Podman)."
    fi

    command -v flatpak >/dev/null 2>&1 || error "flatpak is not installed. Install it with: sudo apt install flatpak"
    command -v flatpak-builder >/dev/null 2>&1 || error "flatpak-builder is not installed. Install it with: sudo apt install flatpak-builder"

    if [[ ! -f "${MANIFEST}" ]]; then
        error "Manifest not found: ${MANIFEST}"
    fi

    info "All prerequisites met."
}

install_runtimes() {
    info "Installing required Flatpak runtimes and SDK extensions..."

    flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

    info "Installing GNOME Platform/SDK ${RUNTIME_VERSION}..."
    flatpak install -y flathub org.gnome.Platform//${RUNTIME_VERSION} org.gnome.Sdk//${RUNTIME_VERSION} || \
        warn "GNOME Platform/SDK installation may have issues. Continuing..."

    info "Installing Rust stable extension..."
    flatpak install -y flathub org.freedesktop.Sdk.Extension.rust-stable//${FDK_VERSION} || \
        warn "Rust extension installation may have issues. Continuing..."

    info "Installing Node.js 22 extension..."
    flatpak install -y flathub org.freedesktop.Sdk.Extension.node22//${FDK_VERSION} || \
        warn "Node.js extension installation may have issues. Continuing..."

    info "Runtimes and SDK extensions installed."
}

build_flatpak() {
    info "Building Flatpak package..."

    flatpak-builder \
        --force-clean \
        --repo="${REPO_DIR}" \
        "${BUILD_DIR}" \
        "${MANIFEST}"

    info "Flatpak build completed."
}

export_bundle() {
    info "Exporting Flatpak bundle..."

    flatpak build-bundle "${REPO_DIR}" "${BUNDLE_NAME}" "${APP_ID}"

    info "Bundle exported: ${BUNDLE_NAME}"
}

install_locally() {
    info "Installing Flatpak locally..."

    flatpak-builder --user --install --force-clean "${BUILD_DIR}" "${MANIFEST}"

    info "Installed locally. Run with: flatpak run ${APP_ID}"
}

clean() {
    info "Cleaning build artifacts..."

    rm -rf "${BUILD_DIR}" "${REPO_DIR}"

    info "Clean completed."
}

show_usage() {
    cat <<EOF
Happy Friday - Flatpak Build Script
====================================

Usage: $0 <command>

Commands:
  build       Build the Flatpak package and export a .flatpak bundle
  install     Build and install the Flatpak locally (user-level)
  clean       Remove build artifacts
  full        Install runtimes, build, and export bundle (full pipeline)

Options:
  -h, --help  Show this help message

Examples:
  $0 full           # First-time setup: install runtimes + build + export
  $0 build          # Build and export bundle (assumes runtimes installed)
  $0 install        # Build and install locally for testing
  $0 clean          # Clean build artifacts

Note: This script must be run on a Linux system with Flatpak support.
      If you are on macOS, use a Linux VM or container.

EOF
}

main() {
    local command="${1:-}"

    case "${command}" in
        build)
            check_prerequisites
            build_flatpak
            export_bundle
            info "Build complete! Bundle: ${BUNDLE_NAME}"
            ;;
        install)
            check_prerequisites
            install_locally
            ;;
        clean)
            clean
            ;;
        full)
            check_prerequisites
            install_runtimes
            build_flatpak
            export_bundle
            info "Full pipeline complete! Bundle: ${BUNDLE_NAME}"
            ;;
        -h|--help|help)
            show_usage
            ;;
        *)
            error "Unknown command: ${command:-<none>}. Run '$0 --help' for usage."
            ;;
    esac
}

main "$@"
