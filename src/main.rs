mod code_grep_results;
mod cache_handler;
mod cli;

use std::{
    env,
    fs,
    error::Error,
    path::PathBuf,
};
use futures_lite::{
    io::BufReader,
    prelude::*,
};
use async_process::{
    Command,
    Stdio,
};
use async_io;

use crate::code_grep_results::GrepRes;
use crate::cache_handler::Cache;

fn open_vim(selected_element: Option<&GrepRes>) -> Result<(), Box<dyn Error>> {
    use std::process::Command;

    if let Some(selected) = selected_element {
        Command::new("vim")
            .arg(format!("+{}", selected.getl()))
            .arg(fs::canonicalize(&PathBuf::from(selected.getp()))?)
            .spawn()?
            .wait()?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    let home_dir_path = env::home_dir().ok_or("Error: Cannot find home directory")?;
    if let Some(c) = Cache::new(home_dir_path) {
        c.cache_history(&args.join(" "))?;
        if cli::check_for_history() {
            c.print_history()?;
            return Ok(())
        }
    };

    if cli::check_for_help() {
        cli::print_help();
        return Ok(())
    }

    let res_vec = async_io::block_on(async {

        let mut grep = Command::new("grep")
            .args(args)
            .arg("-rn")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Error: Failed to spawn grep command");

        let lines = BufReader::new(grep.stdout.take().unwrap()).lines();
        GrepRes::deserialize_output(lines).await
    })?;

    if res_vec.len() > 0 {
        open_vim(res_vec.get(cli::select_output()))?;
    }

    Ok(())
}
