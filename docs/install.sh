#!/usr/bin/env sh
set -eu

GITHUB_REPO="DezBenedek/d"
BINARY_NAME="d"

detect_asset_name() {
  os_name="$(uname -s)"
  arch_name="$(uname -m)"

  case "$os_name" in
    Darwin)
      case "$arch_name" in
        arm64) echo "d-macos-arm64" ;;
        x86_64) echo "d-macos-x64" ;;
        *) echo "Ismeretlen macOS architektura: $arch_name" >&2; exit 1 ;;
      esac
      ;;
    Linux)
      case "$arch_name" in
        x86_64) echo "d-linux-x64" ;;
        *) echo "Ismeretlen Linux architektura: $arch_name" >&2; exit 1 ;;
      esac
      ;;
    *)
      echo "Nem tamogatott operacios rendszer: $os_name" >&2
      exit 1
      ;;
  esac
}

main() {
  asset_name="$(detect_asset_name)"
  download_url="https://github.com/${GITHUB_REPO}/releases/latest/download/${asset_name}"

  if [ "$(uname -s)" = "Darwin" ] && [ "$(uname -m)" = "arm64" ]; then
    install_dir="/opt/homebrew/bin"
  else
    install_dir="/usr/local/bin"
  fi

  echo "Letoltes: $download_url"
  tmp_path="$(mktemp)"
  curl -fsSL -o "$tmp_path" "$download_url"
  chmod +x "$tmp_path"

  mkdir -p "$install_dir"
  mv "$tmp_path" "$install_dir/$BINARY_NAME"

  echo "Telepitve: $install_dir/$BINARY_NAME"
  echo "Probald ki: $BINARY_NAME ip"
}

main
