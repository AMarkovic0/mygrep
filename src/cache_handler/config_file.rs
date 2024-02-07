use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct ConfigFile {
    path: PathBuf,
}

impl ConfigFile {
    pub fn new(mut fpath: PathBuf, fname: String) -> std::io::Result<ConfigFile> {
        fpath.push(fname);

        if !fpath.exists() {
            File::create(fpath.to_str().expect("ERROR: Failed getting path to home dir"))?;
        }

        Ok(ConfigFile {
            path: fpath,
        })
    }

    fn get_path_str(&self) -> std::io::Result<&str> {
        match self.path.to_str() {
            Some(s) => Ok(s),
            None => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Path str not found"))
        }
    }

    pub fn read(&self) -> std::io::Result<String> {
        let mut contents = String::new();

        File::open(self.get_path_str()?)?
            .read_to_string(&mut contents)?;

        Ok(contents)
    }

    pub fn write_line(&self, line: &String) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(String::from(self.get_path_str()?))?;

        if let Err(e) = writeln!(file, "{}", &line) {
            return Err(e)
        }

        Ok(())
    }

    pub fn print(&self) -> std::io::Result<()> {
        println!("{}", self.read()?);
        Ok(())
    }
}
