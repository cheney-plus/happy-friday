#!/bin/bash
set -e

APP_ID="com.cheney.happy-friday"
MANIFEST="com.cheney.happy-friday.yml"

echo "=== Building $APP_ID as Flatpak ==="
echo ""

if [ ! -f "$MANIFEST" ]; then
    echo "Error: Manifest file '$MANIFEST' not found!"
    exit 1
fi

echo "Step 1: Adding flathub remote for user..."
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo

echo "Step 2: Installing dependencies..."
flatpak install --user -y flathub org.gnome.Platform//47 \
    org.gnome.Sdk//47 \
    org.freedesktop.Sdk.Extension.rust-stable//24.08 \
    org.freedesktop.Sdk.Extension.node22//24.08 \
    2>/dev/null || true

echo ""
echo "Step 3: Building Flatpak..."
flatpak-builder --user --install-deps-from=flathub --force-clean _build "$MANIFEST"

echo ""
echo "Step 4: Exporting bundle..."
flatpak-builder --user --repo=repo --force-clean _build "$MANIFEST"

echo ""
echo "Step 5: Creating flatpak bundle..."
flatpak build-bundle repo "${APP_ID}.flatpak" "$APP_ID"

echo ""
echo "=== Build complete! ==="
echo ""
echo "To install and run:"
echo "  flatpak install --user -y --reinstall ${APP_ID}.flatpak"
echo "  flatpak run $APP_ID"
echo ""
echo "Bundle file: ${APP_ID}.flatpak"
