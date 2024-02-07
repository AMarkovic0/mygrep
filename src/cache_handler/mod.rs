mod config_file;

use std::path::PathBuf;
use crate::cache_handler::config_file::ConfigFile;

pub struct Cache {
    config: ConfigFile,
    history: ConfigFile
}

impl Cache {
    pub fn new(mut home_dir: PathBuf) -> Option<Cache> {
        if home_dir.exists() {
            home_dir.push(".pgrep");

            return Some(Cache {
                config: ConfigFile::new(
                    home_dir.clone(),
                    "config.toml".to_string()
                ).expect("ERROR: Failed to create config file"),
                history: ConfigFile::new(
                    home_dir.clone(),
                    "history.log".to_string()
                ).expect("ERROR: Failed to create history file"),
            })
        }

        None
    }

    pub fn cache_history(&self, args: &String) -> std::io::Result<()> {
        self.history.write_line(args)?;
        Ok(())
    }

    pub fn print_history(&self) -> std::io::Result<()> {
        self.history.print()?;
        Ok(())
    }
}
