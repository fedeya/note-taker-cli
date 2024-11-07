use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        content: String,

        #[arg(short, long)]
        category: Option<String>,
    },
    List {
        category: Option<String>,
    },
}
