use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg()]
    pub input: String,
}

pub fn get_bin_name() -> String {
    let args: Vec<String> = std::env::args().collect();
    let basename = Path::new(&args[0]).file_stem().unwrap();

    basename.to_str().unwrap().to_string()
}

pub fn input_iterator<F: Fn(&str)>(execute: F) {
    let input_filename = format!("inputs/{}.txt", get_bin_name());

    if let Ok(lines) = std::fs::read_to_string(input_filename) {
        for line in lines.split('\n') {
            execute(line);
        }
    }
}
