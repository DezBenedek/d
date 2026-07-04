#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RELEASE_DIR="$SCRIPT_DIR/release"

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
  if ! command -v gh >/dev/null 2>&1; then
    echo "Hiányzik a GitHub CLI. Telepítsd: brew install gh" >&2
    exit 1
  fi
}

check_release_dir_not_empty() {
  if [[ ! -d "$RELEASE_DIR" ]] || [[ -z "$(ls -A "$RELEASE_DIR" 2>/dev/null)" ]]; then
    echo "A release/ mappa üres vagy nem létezik. Előbb futtasd: ./build.sh" >&2
    exit 1
  fi
}

main() {
  cd "$SCRIPT_DIR"
  check_required_tools
  check_release_dir_not_empty

  local version tag
  version="$(read_version_from_cargo_toml)"
  tag="v$version"

  echo "Git tag létrehozása és push-olása: $tag"
  git tag "$tag"
  git push origin "$tag"

  echo "GitHub Release létrehozása és a fájlok feltöltése..."
  gh release create "$tag" "$RELEASE_DIR"/* \
    --title "$tag" \
    --generate-notes \
    --verify-tag

  echo ""
  echo "Kész: https://github.com/DezBenedek/d/releases/tag/$tag"
}

main
