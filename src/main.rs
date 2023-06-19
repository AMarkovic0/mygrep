mod code_grep_results;
mod cache_handler;

use std::env;
use std::error::Error;
use std::process::Command;
use std::fs;
use std::path::PathBuf;

use crate::code_grep_results::GrepRes;
use crate::cache_handler::Cache;

macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("ERROR: String expected");
        let $out = inner.trim().parse::<$type>().expect("ERROR: Parsing user input");
    };
}

fn check_for(arg: &str) -> bool {
    let mut ret = false;

    if let Some(_help) = env::args().find(|x| x == arg) {
        ret = true;
    } else if env::args().len() == 1 {
        ret = true;
    }

    ret
}

fn check_for_help() -> bool {
    check_for("--help")
}

fn check_for_history() -> bool {
    check_for("--history")
}

fn print_help() {
    println!("Opens seleced file, on selected line from grep recursive search (grep -rn ...).\n");
    println!("Usage: pgrep [OPTION]... PATTERN [FILE]...
Search for PATTERN in each FILE and opens file on selected location.
Example: pgrep -i --include=*.c 'hello world' main.c

--history Prints history of pgrap call arguments
--help Prints help message
");
    println!("Here is how the grep commands works: \n");
    Command::new("grep")
        .arg("--help")
        .spawn()
        .expect("ERROR: Failed to spwan grep")
        .wait()
        .expect("ERROR: Grep failed to execute");
}

fn deserialize_output(res: String) -> Vec<GrepRes> {
    let mut res_vec = Vec::new();
    let mut index = 0;

    for r in res.split("\n").collect::<Vec<&str>>() {
        if let Some(gres) = GrepRes::new(r) {
            gres.print(index);
            res_vec.push(gres);
            index += 1;
        }
    }

    res_vec
}

fn select_output() -> usize {
    read!(selected as usize);
    selected
}

fn open_vim(selected_element: Option<&GrepRes>) {
    if let Some(selected) = selected_element {
        Command::new("vim")
            .arg(format!("+{}", selected.getl()))
            .arg(fs::canonicalize(&PathBuf::from(selected.getp()))
                .expect("ERROR: Selected path does not exists"))
            .spawn()
            .expect("ERROR: Vim opening failed")
            .wait()
            .expect("ERROR: Vim execution failed");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    if let Some(c) = Cache::new() {
        c.cache_history(&args.join(" "))?;
        if check_for_history() {
            c.print_history()?;
            return Ok(())
        }
    };

    if check_for_help() {
        print_help();
        return Ok(())
    }

    let res = Command::new("grep")
        .args(args)
        .arg("-rn")
        .output()
        .expect("Error: grep command failed to execute");

    let res = String::from_utf8(res.stdout)
        .expect("ERROR: Cannot convert grep output to string");
    let res_vec = deserialize_output(res);

    if res_vec.len() > 0 {
        open_vim(res_vec.get(select_output()));
    }

    Ok(())
}
