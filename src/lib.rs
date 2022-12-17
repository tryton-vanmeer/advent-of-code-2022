use std::path::Path;

pub fn get_input_file() -> anyhow::Result<Vec<String>> {
    let args: Vec<String> = std::env::args().collect();
    let basename = Path::new(&args[0]).file_stem().unwrap().to_str().unwrap();
    let filename = format!("inputs/{}.txt", basename);

    let lines = std::fs::read_to_string(filename)?
        .split('\n')
        .map(str::to_string)
        .collect();

    Ok(lines)
}
