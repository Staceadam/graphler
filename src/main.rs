use walkdir::WalkDir;
use std::fs;

fn main() {
    const PATH: &str = "src/etc";

    for file in WalkDir::new(PATH).into_iter().filter_map(|file| file.ok()) {
        //probably a less gross way to write this...
        //check if its a file and has a .graphql extension
        if file.metadata().unwrap().is_file() && file.file_name().to_str().map(|s| s.ends_with(".graphql")).unwrap_or(false) {
            let path = file.path();
            println!("graphql file path {}", path.display());
            let data = fs::read_to_string(path).expect("Unable to read file");
            println!("contents {}", data);
        }
    }
}

