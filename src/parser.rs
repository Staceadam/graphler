use graphql_parser::parse_query;
use graphql_parser::query::Field;
use std::fs;
use std::io::Error;
use std::path::Path;
use walkdir::WalkDir;
use crate::collection::{Collection, Query};
use crate::visitor::Visit;


pub fn parse(path: &str) -> Result<Collection, Error> {
    let mut collection = Collection::new("insertNameFromCliInputOrUrlBase");
    //TODO: could do the extension match in the filter_map here
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        let extension = Path::new(file.file_name()).extension();
        match extension {
            Some(val) => {
                let ext = val
                    .to_str()
                    .expect("failed to get file with that extension");
                if ext == "graphql" || ext == "gql" {
                    let data = fs::read_to_string(file.path()).expect("Unable to read file");
                    let ast = parse_query(data.as_str()).expect("failed to parse the file");
                    for f in ast.visit::<Field<_>>() {
                        let query = Query::new(f, &ast);
                        // if f.arguments.len() > 0 {
                        // println!("============================");
                        //     println!("{:#?}", f);
                        // }
                        collection.item.push(query)
                    }
                }
            }
            None => println!("Couldn't find any .graphql or .gql files in this project"),
        }
    }
    Ok(collection)
}
