#!/usr/bin/env bash
set -euo pipefail

BINARY_NAME="d"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if ! command -v cargo >/dev/null 2>&1; then
  echo "Nincs telepitve a Rust (cargo). Telepitsd innen: https://rustup.rs"
  exit 1
fi

echo "Forditas release modban (meret-optimalizalt profillal)..."
cd "$SCRIPT_DIR"
cargo build --release

INSTALL_DIR="$HOME/.local/bin"

if [[ ! -d "$INSTALL_DIR" ]]; then
  echo "A cel mappa ($INSTALL_DIR) nem letezik, letrehozom..."
  mkdir -p "$INSTALL_DIR"
fi

cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "Telepitve: $INSTALL_DIR/$BINARY_NAME"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo ""
  echo "FIGYELEM: a $INSTALL_DIR nincs a PATH-odon, ezert a '$BINARY_NAME' parancs meg nem lesz elerheto."
  echo "Add hozza a shell konfigodhoz (pl. ~/.zshrc):"
  echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
  echo "Majd toltsd ujra: source ~/.zshrc"
else
  echo "Kesz! Probald ki: $BINARY_NAME ip"
fi
