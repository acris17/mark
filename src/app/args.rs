use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Open the mark
    Open {
        /// The name of the mark
        name: String,
    },
    /// Edit the mark file
    Edit,
    /// Get the mark path
    Get {
        /// The name of the mark
        name: String,
    },
}
