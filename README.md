# d CLI

A small, fast command-line toolkit for everyday developer tasks on macOS — git workflow helpers, GitHub repo setup, local networking, secret generation, and self-updates.

## Why d?

Instead of juggling several one-off scripts and long git/gh invocations, `d` wraps the most common actions into short, memorable commands:

- **One-shot commit & push** — `d push your message here`
- **Interactive GitHub repo setup** — create or overwrite a repo with backup of old code
- **Self-updating binary** — `d update` pulls the latest GitHub release
- **Multilingual UI** — auto-detects your system language (English by default)

Built in Rust with a size-optimized release profile.

## Install

```bash
curl -fsSL https://dcli.dezso.hu/install.sh | bash
```

Verify:

```bash
d version
# or
d --help
```

You can also download binaries from the [GitHub Releases](https://github.com/DezBenedek/d/releases) page (`d` binary or `d-installer.pkg`).

## Features

### Quick git push

Stages everything, commits, and pushes the current branch in one step:

```bash
d push add login screen
```

### Git helpers

| Command | What it does |
|---------|----------------|
| `d git setup` | Interactive setup: ensures `gh` is installed/authenticated, configures `user.name` / `user.email`, creates a public or private GitHub repo (optional org), or overwrites an existing one after moving old remote code to a `backup/pre-setup-*` branch |
| `d git fix` | Untracks files that are listed in `.gitignore` but still tracked in the index |
| `d git update` | `git pull origin <current-branch>` |

### Self-update

Downloads the latest release asset from GitHub and replaces the running binary:

```bash
d update
```

### Local IP

Prints your machine’s LAN IP (useful for local servers / device testing):

```bash
d ip
```

### Secret / token generators

Thin wrappers around OpenSSL:

```bash
d gen hex        # 32 bytes → hex
d gen hex 64
d gen base64     # 32 bytes → base64
d gen base64 64
```

### macOS defaults

Apply a small set of sensible Finder / menu bar defaults:

```bash
d macos start
```

Enables battery percentage, path bar, status bar, and hidden files, then restarts Finder / Control Center.

### Multilingual UI

Supported languages: **en** (default), **hu**, **de**, **es**, **it**, **zh**, **ru**, **uk**.

Detection order:

1. `--lang` / `-L`
2. `D_LANG` environment variable
3. `LC_ALL` → `LC_MESSAGES` → `LANG`
4. macOS `AppleLocale`
5. Fallback: English

```bash
d --lang hu
d -L de ip
D_LANG=zh d git --help
```

Help text, prompts, and command messages are all localized.

## Command reference

| Command | Description |
|---------|-------------|
| `d` / `d help` | Show help (same as `d --help`) |
| `d version` | Print CLI version |
| `d ip` | Print local (LAN) IP address |
| `d push <message…>` | `git add -A` + commit + push current branch |
| `d update` | Install latest version from GitHub Releases |
| `d git setup` | Interactive git + GitHub repo setup via `gh` |
| `d git fix` | Remove ignore-but-tracked files from the index |
| `d git update` | Pull latest changes for the current branch |
| `d gen hex [n]` | Random hex (`openssl rand -hex`, default `n=32`) |
| `d gen base64 [n]` | Random base64 (`openssl rand -base64`, default `n=32`) |
| `d macos start` | Apply initial macOS Finder / menu bar tweaks |
| `--lang` / `-L` | Force UI language |
| `--authors` | Print author name |
| `--doc` | Print documentation URL |

## Examples

```bash
# Everyday workflow
d push fix typo in readme
d git update
d update

# New project → GitHub
cd my-app
d git setup

# Cleanup after editing .gitignore
d git fix
d push untrack ignored files

# Utilities
d ip
d gen hex 64
d macos start
```

## Development

Requires a Rust toolchain ([rustup](https://rustup.rs)).

```bash
cargo build --release
cargo run -- --help
./build-pkg.sh          # builds d-installer.pkg for macOS
```

CLI parsing uses [`clap`](https://crates.io/crates/clap).

## Info

- **Version:** 1.0.0
- **Author:** Dezső Benedek
- **Repo / docs:** [github.com/DezBenedek/d](https://github.com/DezBenedek/d)
- **License:** MIT
