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
}

#[derive(Subcommand)]
pub enum GitCommands {
    /// A .gitignore által tiltott, de már trackelt fájlok eltávolítása a git indexből
    Fix,
    /// git-hez kapcsolódó frissítési művelet
    Update,
}
