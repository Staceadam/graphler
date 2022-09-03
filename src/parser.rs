use graphql_parser::{parse_query, query};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, to_writer_pretty};
use std::fs;
use std::fs::File;
use std::io::Error;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
struct Url {
    raw: String,
    protocol: String,
    host: Vec<String>,
    path: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Graphql {
    query: String,
    variables: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Body {
    mode: String,
    graphql: Graphql,
}
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    header: Vec<String>,
    body: Body,
    url: Url,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    name: String,
    request: Request,
    response: Vec<String>,
}

pub fn parse_to_ast<'a>(data: &'a String) -> query::Document<&'a str> {
    let ast: query::Document<&str> =
        parse_query::<&str>(data.as_str()).expect("failed to parse the file");
    ast
}

pub fn write_to_file(query: Query, file_name: &str) {
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

pub fn parse(path: &str) -> Result<(), Error> {
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
                    write_to_file(query, "graphler.json");
                }
            }
            None => println!("Couldn't find any .graphql or .gql files in this project"),
        }
    }
    Ok(())
}
