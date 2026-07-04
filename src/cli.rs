use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "d", version, about = "d CLI - by Dezső Benedek Péter")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
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
}
