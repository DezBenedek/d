#!/usr/bin/env bash
set -euo pipefail

BINARY_NAME="d"
IDENTIFIER="com.dezbenedek.d"
VERSION="0.1.0"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STAGE_DIR="$SCRIPT_DIR/pkg-root"
OUTPUT_PKG="$SCRIPT_DIR/d-installer.pkg"

if ! command -v pkgbuild >/dev/null 2>&1; then
  echo "A pkgbuild eszkoz hianyzik. Telepitsd az Xcode Command Line Tools-t:"
  echo "  xcode-select --install"
  exit 1
fi

if [[ "$(uname -m)" == "arm64" ]]; then
  INSTALL_DIR="/opt/homebrew/bin"
else
  INSTALL_DIR="/usr/local/bin"
fi

echo "Forditas release modban..."
cd "$SCRIPT_DIR"
cargo build --release

echo "Staging mappa osszeallitasa ($STAGE_DIR)..."
rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR$INSTALL_DIR"
cp "target/release/$BINARY_NAME" "$STAGE_DIR$INSTALL_DIR/$BINARY_NAME"
chmod +x "$STAGE_DIR$INSTALL_DIR/$BINARY_NAME"

echo "Pkg osszeallitasa..."
pkgbuild \
  --root "$STAGE_DIR" \
  --identifier "$IDENTIFIER" \
  --version "$VERSION" \
  --install-location "/" \
  "$OUTPUT_PKG"

rm -rf "$STAGE_DIR"

echo ""
echo "Kesz: $OUTPUT_PKG"
echo "Alairatlan csomag, ezert az elso megnyitasnal:"
echo "  jobb-katt a pkg-n -> Megnyitas -> Megnyitas megerositese"
echo "vagy System Settings -> Privacy & Security -> Open Anyway."
