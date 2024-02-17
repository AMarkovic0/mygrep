use std::str::FromStr;
use std::error::Error;

use futures_lite::{
    io::{
        Lines,
        BufReader,
    },
    stream::StreamExt,
};
use async_process::ChildStdout;
use regex::Regex;
use colored::*;

#[derive(Debug)]
pub struct GrepRes {
    path: String,
    line: u32,
    text: String,
}

impl GrepRes {
    pub fn new(s: String) -> Option<GrepRes> {
        match Regex::new(r"^([^:]+):(\d+):\s*(.*)").ok()?.captures(&s) {
            Some(captures) => Some(GrepRes {
                path: captures[1].to_string(),
                line: FromStr::from_str(&captures[2]).unwrap(),
                text: captures[3].to_string(),
            }),
            None => {
                None
            }
        }
    }

    pub fn getl(&self) -> u32 {
        self.line
    }

    pub fn getp(&self) -> &String {
        &self.path
    }

    pub fn gett(&self) -> &String {
        &self.text
    }

    pub fn print(&self, index: usize) {
        let s: String = format!(
            "[{}] {}:{}:",
            index,
            self.getp().purple(),
            self.getl().to_string().green()
        );
        println!("{} {}", s.blue(), self.gett());
    }

    pub async fn deserialize_output(mut lines: Lines<BufReader<ChildStdout>>) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut index = 0;
        let mut res_vec = Vec::new();

        while let Some(line) = lines.next().await {
            if let Some(gres) = GrepRes::new(line?) {
                gres.print(index);
                res_vec.push(gres);
                index += 1;
            }
        }

        Ok(res_vec)
    }
}
