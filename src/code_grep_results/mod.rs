use std::str::FromStr;

use regex::Regex;
use colored::*;

#[derive(Debug)]
pub struct GrepRes {
    path: String,
    line: u32,
    text: String,
}

impl GrepRes {
    pub fn new(s: &str) -> Option<GrepRes> {
        match Regex::new(r"^([^:]+):(\d+):\s*(.*)").ok()?.captures(s) {
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

    pub fn deserialize_output(res: String) -> Vec<Self> {
        let mut res_vec = Vec::new();

        for (index, r) in res.split("\n").collect::<Vec<&str>>().iter().enumerate() {
            if let Some(gres) = GrepRes::new(r) {
                gres.print(index);
                res_vec.push(gres);
            }
        }

        res_vec
    }
}
