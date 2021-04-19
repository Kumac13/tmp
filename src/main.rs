mod cli;
mod file;

use clap::Clap;
use cli::CommandLineArgs;
use file::{Config, FileManagement};
use std::io;

fn main() -> io::Result<()> {
    let cmd: CommandLineArgs = CommandLineArgs::parse();

    // Check config file.
    let config = Config::default();
    if config.is_file_missing() {
        if !config.file_exists() {
            config.file_create()?;
            println!("Created config file");
        }
        if config.file_read().is_err() {
            config.setup_config_file()?;
        }
        println!("First time setup coplete.");
        Ok(())
    } else {
        let file_path = config.file_read().unwrap();
        println!("{:#?}", cmd);
        println!("{:#?}", file_path);

        Ok(())
    }
}
