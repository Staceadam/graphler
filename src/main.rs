use walkdir::WalkDir;
use serde_json::{json,Value, to_writer_pretty};
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
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
                    // format
                    // transform ast to json

                    let json_template: Value = json!({
                        "name": "character",
                        "request": {
                            "method": "POST",
                            "header": [],
                            "body": {
                                "mode": "graphql",
                                "graphql": {
                                    "query": format!("{}", ast),
                                    "variables": "{\n\t\"id\": \"0\"\n}"
                                }
                            },
                            "url": {
                                "raw": "https://rickandmortyapi.com/graphql",
                                "protocol": "https",
                                "host": [
                                    "rickandmortyapi",
                                    "com"
                                ],
                                "path": [
                                    "graphql"
                                ]
                            }
                        },
                        "response": []
                    });




                    // write to file
                    let file = File::create("collection.json").unwrap();
                    to_writer_pretty(file, &json_template).unwrap();
                }
            }
            None => ()
        }
    }
}

