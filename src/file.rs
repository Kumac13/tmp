use dialoguer::theme::ColorfulTheme;
use dialoguer::*;
use std::io::{ErrorKind, Read, Write};
use std::{env, fs, io, path};

pub trait FileManagement {
    fn file_create(&self) -> io::Result<()>;
    fn file_exists(&self) -> bool;
    fn file_read(&self) -> io::Result<String>;
    fn file_write(&self, value: String) -> io::Result<()>;
    fn file_rm(&self) -> io::Result<()>;
    fn is_file_missing(&self) -> bool;
}

pub struct Config;

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

impl FileManagement for Config {
    fn file_create(&self) -> io::Result<()> {
        let config_path = self.config_path_for()?;
        let config_file = fs::File::create(config_path);
        Ok(())
    }
    fn file_exists(&self) -> bool {
        self.config_path_for().and_then(fs::metadata).is_ok()
    }

    fn file_read(&self) -> io::Result<String> {
        let config_path = self.config_path_for()?;
        // Make sure file exists
        fs::metadata(&config_path)?;

        let mut file = fs::File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.is_empty() {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("File is empty: {}", &config_path),
            ));
        } else if contents.ends_with('\n') {
            contents.pop().ok_or_else(|| {
                io::Error::new(ErrorKind::Other, "Unable to remove last char from file")
            })?;
        }
        Ok(contents)
    }

    fn file_write(&self, value: String) -> io::Result<()> {
        let config_path = self.config_path_for()?;
        let mut file = fs::File::create(&path::Path::new(config_path.as_str()))?;
        file.write_all(value.as_bytes())
    }

    fn file_rm(&self) -> io::Result<()> {
        let config_path = self.config_path_for()?;

        // Make sure file exists
        fs::metadata(&config_path)?;
        fs::remove_file(&config_path)
    }
    fn is_file_missing(&self) -> bool {
        self.file_read().is_err()
    }
}

impl Config {
    pub fn setup_config_file(&self) -> io::Result<()> {
        let theme = ColorfulTheme::default();

        let input = Input::with_theme(&theme)
            .with_prompt("Absolute path to your temporary directory")
            .interact()?;

        self.file_write(input)
    }

    pub fn config_path_for(&self) -> io::Result<String> {
        env::home_dir()
            .map(|home| format!("{}/{}", home.display(), ".tmprc"))
            .ok_or_else(|| {
                io::Error::new(
                    ErrorKind::NotFound,
                    "Could not resolve your $HOME directory",
                )
            })
    }
}
