mod config_file;

use std::fs;
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

            let _ = Cache::create_config_dir(&home_dir)
                .expect("ERROR: Failed to create config dir");

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

    fn create_config_dir(home_dir: &PathBuf) -> std::io::Result<()>{
        if !home_dir.exists() {
            if let Some(dir_path) = home_dir.to_str() {
                let _ = fs::create_dir(dir_path);
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Config directory path str not found"
                ));
            }

        }

        return Ok(());
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
