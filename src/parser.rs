use graphql_parser::parse_query;
use graphql_parser::query::Field;
use std::fs;
use std::io::Error;
use std::path::Path;
use walkdir::WalkDir;
use crate::collection::{Collection, Query};
use crate::visitor::Visit;


pub fn parse(path: &str) -> Result<Collection, Error> {
    let mut collection = Collection::new(
        "insertNameFromCliInputOrUrlBase", 
        "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
    );
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
                    let visitor = ast.visit();
                    visitor.visit_field(x| collection.item.push(Query::new(x)))
                    for f in ast.visit() {
                        println!("{:#?}", f);
                        if f.selection_set.items.len() > 0 && f.position.line == 2 {
                            let query = Query::new(f, &ast);
                            collection.item.push(query)
                        }
                    }
                }
            }
            None => (),
        }
    }
    Ok(collection)
}
