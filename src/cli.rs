use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "d", version, about = "Sajat parancs-csalad macOS-hez")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// A gep helyi (LAN) IP-cimenek kiirasa
    Ip,
    /// A legujabb verzio letoltese es telepitese (github.com/DezBenedek/d)
    Update,
}
