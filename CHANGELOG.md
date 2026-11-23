# Changelog

## [1.5.0] - 2026-11-23

### Added
- `d port <n>` — show what is listening on a TCP port (via `lsof`)
- `d killport <n>` — kill the process listening on a TCP port (TERM, then KILL if needed)
- `d serve [dir] [--port]` — serve a folder over HTTP for local testing (via `python3`, prints localhost + LAN URL)
- `d sys` — system info: OS, CPU, memory, disk free, uptime, LAN IP
- `d git clean` — untrack ignored-but-tracked files, delete merged branches, prune origin (with confirmation)

## [1.4.0] - 2026-11-23

### Added
- `d download music` now accepts **Apple Music** links (`music.apple.com`, `itunes.apple.com`): reads the track's artist/title from Apple's public catalog (no Apple ID needed) and downloads the match from YouTube via `spotdl`, same as Spotify

### Added
- `d install yt-dlp` — installs `yt-dlp` and `ffmpeg` via Homebrew with no prompt
- `d download youtube` auto-installs the same tools if they are missing
- Video quality picker for `d download youtube`: 270p, 480p, 720p, 1080p, 1440p (`--quality`)
- `d download music` — audio from YouTube, YouTube Music, or Spotify (128k–320k; Spotify via `spotdl`/`pipx`)

### Changed
- YouTube help, README, and runtime disclaimer are always English: **For educational purposes only.**

## [1.2.0] - 2026-08-22

### Added
- `d download youtube [url]` — educational YouTube downloader via `yt-dlp`, with a native macOS folder picker and an educational-purpose disclaimer

## [1.1.0] - 2026-08-16

### Added
- Unit tests for language parsing, yes/no/visibility helpers, `trf` placeholders, semver/checksum parsing, UUID v4 and password generation
- CI workflow (fmt, clippy, test, release build) on `stable` and MSRV `1.85`
- Release workflow: tagged `v*` builds publish `d`, `d.sha256`, and `d-installer.pkg` to GitHub Releases
- `d update` skips download when already current, verifies SHA-256, and retries with `sudo` on permission errors
- `d gen uuid` and `d gen password [n]`
- `d macos dock`, `d macos flushdns`, `d macos reset`
- MIT `LICENSE`, Cargo.toml crate metadata, `rust-version = "1.85"`, `rust-toolchain.toml`
- `Cargo.lock` committed for reproducible builds

### Changed
- Hungarian `d git update` help text now matches the other languages
- `.gitignore` ignores `.DS_Store` and `*.pkg`; installer package is no longer versioned

### Fixed
- `rustfmt` formatting and clippy `collapsible_if` warning

## [1.0.0] - 2026-08-11

### Added
- Full multilingual UI with auto-detect (en default): Hungarian, German, Spanish, Italian, Chinese, Russian, Ukrainian
- `--lang` / `-L` and `D_LANG` overrides
- Localized help, prompts, and command messages

## [0.5.0] - 2026-08-11

### Added
- `d version` — verziószám kiírása
- `d` (argumentum nélkül) — ugyanaz a help, mint a `d --help`

## [0.4.4] - 2026-08-11

### Added
- `d git setup` — interaktív git + GitHub repo setup (`gh` CLI-vel): repo létrehozás, config, org, meglévő repo felülírása backup branchel

## [0.4.3] - 2026-07-27

### Changed
- `d gen secret` → `d gen hex` (opcionális hossz, alapértelmezett: 32)

### Added
- `d gen base64` — véletlenszerű base64 (`openssl rand -base64`, alapértelmezett: 32 byte)
- `d gen base64 <n>` / `d gen hex <n>` — tetszőleges hossz

## [0.4.2] - 2026-07-27

### Added
- `d gen secret` — véletlenszerű hex secret generálása (`openssl rand -hex 32`)
- `d gen secret <n>` — tetszőleges hossz, pl. `d gen secret 64` → `openssl rand -hex 64`

## [0.4.1] - 2026-07-05

### Added
- `d macos start` — kezdeti macOS-beállítások

## [0.4.0] - 2026-07-05

### Added
- Stabil kiadás: IP, push, update, git fix/update
