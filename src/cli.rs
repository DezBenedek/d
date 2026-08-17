use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "d", version, about = "d CLI — by Dezső Benedek Péter")]
pub struct Cli {
    /// Print the author name
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub authors: bool,

    /// Print the documentation URL
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub doc: bool,

    /// UI language (en, hu, de, es, it, zh, ru, uk)
    #[arg(long, short = 'L', global = true, value_name = "LANG")]
    pub lang: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print the machine's local (LAN) IP address
    Ip,
    /// Print the CLI version
    Version,
    /// Download and install the latest version (github.com/DezBenedek/d)
    Update,
    /// git add -A + commit + push to the current branch
    Push {
        /// Commit message (multiple words are joined automatically)
        #[arg(trailing_var_arg = true)]
        message: Vec<String>,
    },
    /// Git helpers (fix, setup, update)
    Git {
        #[command(subcommand)]
        command: GitCommands,
    },
    /// macOS-specific settings (start, dock, flushdns, reset)
    Macos {
        #[command(subcommand)]
        command: MacosCommands,
    },
    /// Generators (hex, base64, uuid, password)
    Gen {
        #[command(subcommand)]
        command: GenCommands,
    },
}

#[derive(Subcommand)]
pub enum GitCommands {
    /// Untrack files that are ignored by .gitignore but still tracked
    Fix,
    /// Interactive git + GitHub repo setup (via gh CLI)
    Setup,
    /// Pull the latest changes for the current branch
    Update,
}

#[derive(Subcommand)]
pub enum MacosCommands {
    /// Initial macOS tweaks: battery %, Finder path/status bar, hidden files
    Start,
    /// Auto-hide the Dock
    Dock,
    /// Flush the DNS cache
    Flushdns,
    /// Undo the `d macos start` tweaks
    Reset,
}

#[derive(Subcommand)]
pub enum GenCommands {
    /// Generate random hex (`openssl rand -hex`)
    Hex {
        /// Number of bytes (default: 32)
        #[arg(default_value_t = 32)]
        bytes: u32,
    },
    /// Generate random base64 (`openssl rand -base64`)
    Base64 {
        /// Number of bytes (default: 32)
        #[arg(default_value_t = 32)]
        bytes: u32,
    },
    /// Generate a random UUID v4
    Uuid,
    /// Generate a random password
    Password {
        /// Password length (default: 24)
        #[arg(default_value_t = 24)]
        length: u32,
    },
}
