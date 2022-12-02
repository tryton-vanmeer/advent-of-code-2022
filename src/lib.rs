use std::{
    fs::File,
    io::{self, BufRead},
};

use anyhow::{Ok, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg()]
    pub input: String,
}

pub fn read_lines(filename: String) -> Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
