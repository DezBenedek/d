#!/usr/bin/env bash
set -euo pipefail

BINARY_NAME="d"
IDENTIFIER="com.dezbenedek.d"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RELEASE_DIR="$SCRIPT_DIR/release"
STAGE_DIR="$SCRIPT_DIR/pkg-root"

read_version_from_cargo_toml() {
  local version
  version="$(grep '^version' "$SCRIPT_DIR/Cargo.toml" | head -n1 | cut -d '"' -f2)"
  if [[ -z "$version" ]]; then
    echo "Nem sikerült kiolvasni a verziót a Cargo.toml-ból." >&2
    exit 1
  fi
  echo "$version"
}

check_required_tools() {
  if ! command -v rustup >/dev/null 2>&1; then
    echo "Hiányzik a rustup. Telepítsd innen: https://rustup.rs" >&2
    exit 1
  fi

  if ! command -v brew >/dev/null 2>&1; then
    echo "Hiányzik a Homebrew. Telepítsd innen: https://brew.sh" >&2
    exit 1
  fi

  if ! command -v pkgbuild >/dev/null 2>&1; then
    echo "Hiányzik a pkgbuild. Telepítsd az Xcode Command Line Tools-t:" >&2
    echo "  xcode-select --install" >&2
    exit 1
  fi
}

install_cross_toolchains() {
  echo "Cross-fordító eszközök telepítése (ha még nincsenek meg)..."
  brew install mingw-w64
  brew install filosottile/musl-cross/musl-cross
}

add_rust_targets() {
  echo "Rust cross-compile célok hozzáadása..."
  rustup target add aarch64-apple-darwin
  rustup target add x86_64-apple-darwin
  rustup target add x86_64-unknown-linux-musl
  rustup target add x86_64-pc-windows-gnu
}

write_cargo_linker_config() {
  echo "Cargo linker-konfiguráció írása (.cargo/config.toml)..."
  mkdir -p "$SCRIPT_DIR/.cargo"
  cat > "$SCRIPT_DIR/.cargo/config.toml" << 'EOF'
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
EOF
}

build_macos_universal() {
  echo "macOS build: arm64..."
  cargo build --release --target aarch64-apple-darwin

  echo "macOS build: x64..."
  cargo build --release --target x86_64-apple-darwin

  echo "macOS univerzális bináris összeállítása (lipo)..."
  lipo -create -output "$SCRIPT_DIR/d-universal" \
    "target/aarch64-apple-darwin/release/$BINARY_NAME" \
    "target/x86_64-apple-darwin/release/$BINARY_NAME"
}

build_linux() {
  echo "Linux build: x64 (musl, statikusan linkelve)..."
  cargo build --release --target x86_64-unknown-linux-musl
}

build_windows() {
  echo "Windows build: x64..."
  cargo build --release --target x86_64-pc-windows-gnu
}

build_pkg() {
  local version="$1"

  echo "macOS .pkg összeállítása (univerzális bináris, verzió: $version)..."
  rm -rf "$STAGE_DIR"
  mkdir -p "$STAGE_DIR/usr/local/bin"
  cp "$SCRIPT_DIR/d-universal" "$STAGE_DIR/usr/local/bin/$BINARY_NAME"
  chmod +x "$STAGE_DIR/usr/local/bin/$BINARY_NAME"

  pkgbuild \
    --root "$STAGE_DIR" \
    --identifier "$IDENTIFIER" \
    --version "$version" \
    --install-location "/" \
    "$RELEASE_DIR/d-installer.pkg"

  rm -rf "$STAGE_DIR"
}

copy_binaries_to_release() {
  echo "Binárisok másolása a release/ mappába..."
  mkdir -p "$RELEASE_DIR"

  cp "target/aarch64-apple-darwin/release/$BINARY_NAME" "$RELEASE_DIR/d-macos-arm64"
  cp "target/x86_64-apple-darwin/release/$BINARY_NAME" "$RELEASE_DIR/d-macos-x64"
  cp "target/x86_64-unknown-linux-musl/release/$BINARY_NAME" "$RELEASE_DIR/d-linux-x64"
  cp "target/x86_64-pc-windows-gnu/release/$BINARY_NAME.exe" "$RELEASE_DIR/d-windows-x64.exe"

  chmod +x "$RELEASE_DIR/d-macos-arm64" "$RELEASE_DIR/d-macos-x64" "$RELEASE_DIR/d-linux-x64"
}

main() {
  cd "$SCRIPT_DIR"

  check_required_tools
  local version
  version="$(read_version_from_cargo_toml)"

  install_cross_toolchains
  add_rust_targets
  write_cargo_linker_config

  rm -rf "$RELEASE_DIR"
  mkdir -p "$RELEASE_DIR"

  build_macos_universal
  build_linux
  build_windows
  build_pkg "$version"
  copy_binaries_to_release

  rm -f "$SCRIPT_DIR/d-universal"

  echo ""
  echo "Kész! A release/ mappa tartalma:"
  ls -la "$RELEASE_DIR"
  echo ""
  echo "Kovetkezo lepes: ./publish.sh a GitHub Release letrehozasahoz."
}

main
