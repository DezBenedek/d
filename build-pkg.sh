#!/usr/bin/env bash
set -euo pipefail

BINARY_NAME="d"
IDENTIFIER="com.dezbenedek.d"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STAGE_DIR="$SCRIPT_DIR/pkg-root"
DOCS_DIR="$SCRIPT_DIR/docs"
OUTPUT_PKG="$DOCS_DIR/d-installer.pkg"

if ! command -v pkgbuild >/dev/null 2>&1; then
  echo "A pkgbuild eszköz hiányzik. Telepítsd az Xcode Command Line Tools-t:"
  echo "  xcode-select --install"
  exit 1
fi

VERSION="$(grep '^version' Cargo.toml | head -n1 | cut -d '"' -f2)"
if [[ -z "$VERSION" ]]; then
  echo "Nem sikerült kiolvasni a verziót a Cargo.toml-ból."
  exit 1
fi

if [[ "$(uname -m)" == "arm64" ]]; then
  INSTALL_DIR="/opt/homebrew/bin"
else
  INSTALL_DIR="/usr/local/bin"
fi

echo "Fordítás release módban (verzió: $VERSION)..."
cd "$SCRIPT_DIR"
cargo build --release

echo "Staging mappa összeállítása ($STAGE_DIR)..."
rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR$INSTALL_DIR"
cp "target/release/$BINARY_NAME" "$STAGE_DIR$INSTALL_DIR/$BINARY_NAME"
chmod +x "$STAGE_DIR$INSTALL_DIR/$BINARY_NAME"

echo "Pkg összeállítása..."
mkdir -p "$DOCS_DIR"
pkgbuild \
  --root "$STAGE_DIR" \
  --identifier "$IDENTIFIER" \
  --version "$VERSION" \
  --install-location "/" \
  "$OUTPUT_PKG"

rm -rf "$STAGE_DIR"

echo ""
echo "Kész: $OUTPUT_PKG"
echo "Aláíratlan csomag, ezért az első megnyitásnál:"
echo "  jobb-katt a pkg-n → Megnyitás → Megnyitás megerősítése"
echo "vagy System Settings → Privacy & Security → Open Anyway."
