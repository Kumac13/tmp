use clap::{AppSettings, Clap};
use std::path::PathBuf;

#[derive(Debug, Clap)]
pub enum Action {
    /// List all tmp file
    List,

    /// Delete a tmp file by name.
    Delete {
        #[clap()]
        file_name: String,
    },
}

#[derive(Debug, Clap)]
#[clap(name = "tmp", about = "create a temporary file with Rust")]
#[clap(max_term_width =100, setting = AppSettings::DeriveDisplayOrder)]
pub struct CommandLineArgs {
    #[clap(subcommand)]
    pub action: Action,

    /// Use a different extention
    #[clap(short, long, name = "EXPAND")]
    pub expand: Option<String>,

    /// Use a different file name
    #[clap(short, long, name = "FILE")]
    pub file: Option<PathBuf>,
}
