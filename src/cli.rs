use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "d", version, about = "Sajat parancs-csalad macOS-hez")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Ip,
    Update,
    Push {
        #[arg(trailing_var_arg = true)]
        message: Vec<String>,
    },
}
