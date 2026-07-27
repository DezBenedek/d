use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "d", version, about = "d CLI - by Dezső Benedek Péter")]
pub struct Cli {
    /// A szerző nevének kiírása
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub authors: bool,

    /// A dokumentáció linkjének kiírása
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub doc: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// A gép helyi (LAN) IP-címének kiírása
    Ip,
    /// A legújabb verzió letöltése és telepítése (github.com/DezBenedek/d)
    Update,
    /// git add -A + commit + push az aktuális branch-re
    Push {
        /// A commit üzenet (több szó esetén automatikusan összefűzve)
        #[arg(trailing_var_arg = true)]
        message: Vec<String>,
    },
    /// Git-hez kapcsolódó segédparancsok (fix, update)
    Git {
        #[command(subcommand)]
        command: GitCommands,
    },
    /// macOS-specifikus beallitasok (start, tobbi kesobb)
    Macos {
        #[command(subcommand)]
        command: MacosCommands,
    },
    /// Generáló segédparancsok (secret, többi később)
    Gen {
        #[command(subcommand)]
        command: GenCommands,
    },
}

#[derive(Subcommand)]
pub enum GitCommands {
    /// A .gitignore által tiltott, de már trackelt fájlok eltávolítása a git indexből
    Fix,
    /// git-hez kapcsolódó frissítési művelet
    Update,
}
#[derive(Subcommand)]
pub enum MacosCommands {
    /// Kezdeti macOS-beallitasok: akku szazalek, Finder path/status bar, rejtett fajlok
    Start,
}

#[derive(Subcommand)]
pub enum GenCommands {
    /// Véletlenszerű hex secret generálása (`openssl rand -hex`)
    Secret {
        /// A hex byte-ok száma (alapértelmezett: 32)
        #[arg(default_value_t = 32)]
        bytes: u32,
    },
}
