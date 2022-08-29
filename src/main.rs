use walkdir::WalkDir;
//use serde::{Serialize, Deserialize};
use serde_json::json;
use std::path::Path;
use std::fs;
use graphql_parser::query::parse_query;

fn main() {
    const PATH: &str = "src/etc";
    for file in WalkDir::new(PATH).into_iter().filter_map(|file| file.ok()) {
        let extension = Path::new(file.file_name()).extension();
        match extension {
            Some(val) => {
                let ext = val.to_str().unwrap();
                if ext == "graphql" || ext == "gql" {
                    let path = file.path();
                    let data = fs::read_to_string(path).expect("Unable to read file");
                    // parse graphql query to ast
                    let ast = parse_query::<&str>(data.as_str()).unwrap();
                    // TODO: add visitor pattern here
                    // you can change the following {:?} to {:#?} for different fmting
                    // we are gonna have to parse the files to ASTs because they're
                    // not .js or .json
                    //
                    // is we find/write a visitor pattern to handle this
                    // some options for other graphql crates
                    // https://github.com/graphql-rust/graphql-client
                    let json = json!(format!("{:?}", ast));
                    println!("{}", json);
                }
            }
            None => ()
        }
    }
}

