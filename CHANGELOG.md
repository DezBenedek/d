# Changelog

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
