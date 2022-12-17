use std::path::Path;

// if let Ok(lines) = std::fs::read_to_string(get_input_file()) {
//     for line in lines.split('\n') {
//         // do stuff here
//     }
// }
pub fn get_input_file() -> String {
    let args: Vec<String> = std::env::args().collect();
    let basename = Path::new(&args[0]).file_stem().unwrap().to_str().unwrap();

    format!("inputs/{}.txt", basename)
}
