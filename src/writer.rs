use serde_json::{to_writer_pretty};
use std::fs;
use std::fs::File;
use std::path::Path;
use crate::collection::Collection;

pub fn write_to_file(query: Collection, file_name: &str) {
    let file_check = Path::new(file_name).exists();
    if file_check {
        let file = fs::OpenOptions::new()
            .append(true)
            .open(file_name)
            .expect("couldn't open given file");
        to_writer_pretty(file, &query).expect("failed to write to file");
    }
    let file = File::create(file_name).expect("couldn't create file");
    to_writer_pretty(file, &query).expect("failed to write to file");
}

