use std::path::Path;

pub fn get_input_file() -> String {
    let args: Vec<String> = std::env::args().collect();
    let basename = Path::new(&args[0]).file_stem().unwrap().to_str().unwrap();

    format!("inputs/{}.txt", basename)
}
