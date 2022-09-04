use graphql_parser::{parse_query, query};
use serde_json::{from_value, json};
use std::fs;
use std::io::Error;
use std::path::Path;
use walkdir::WalkDir;
//this is dumb
use crate::collection::{Collection, Query};

pub fn parse_to_ast<'a>(data: &'a String) -> query::Document<&'a str> {
    let ast: query::Document<&str> =
        parse_query::<&str>(data.as_str()).expect("failed to parse the file");
    ast
}

pub fn build_collection_query<'b>(ast: &'b query::Document<&'b str>) -> Query {
    let query: Query = from_value(json!({
        "name": "character",
        "request": {
            "method": "POST",
            "header": [],
            "body": {
                "mode": "graphql",
                "graphql": {
                    "query": format!("{}", &ast),
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
    }))
    .expect("Couldn't read the ast that was passed");
    query
}

pub fn parse(path: &str) -> Result<Collection, Error> {
    let mut collection = Collection::new("insertNameFromCliInputOrUrlBase");
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        let extension = Path::new(file.file_name()).extension();
        match extension {
            Some(val) => {
                let ext = val
                    .to_str()
                    .expect("failed to get file with that extension");
                if ext == "graphql" || ext == "gql" {
                    let data = fs::read_to_string(file.path()).expect("Unable to read file");
                    let ast = parse_to_ast(&data);
                    let query = build_collection_query(&ast);
                    collection.item.push(query)
                }
            }
            None => println!("Couldn't find any .graphql or .gql files in this project"),
        }
    }
    Ok(collection)
}
