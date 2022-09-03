use std::io::Error;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use serde_json::{json, to_writer_pretty, from_value};
use std::path::Path;
use std::fs;
use std::fs::File;
use graphql_parser::query::parse_query;

#[derive(Serialize, Deserialize, Debug)]
struct Url {
    raw: String,
    protocol: String,
    host: Vec<String>,
    path: Vec<String>
}
#[derive(Serialize, Deserialize, Debug)]
struct Graphql {
    query: String,
    variables: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Body {
    mode: String,
    graphql: Graphql
}
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    header: Vec<String>,
    body: Body,
    url: Url
}
#[derive(Serialize, Deserialize, Debug)]
struct Query {
    name: String,
    request: Request,
    response: Vec<String>

}

pub fn parse(path: &str) -> Result<(), Error> {
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        let extension = Path::new(file.file_name()).extension();
        match extension {
            Some(val) => {
                let ext = val.to_str().expect("failed to get file with that extension");
                if ext == "graphql" || ext == "gql" {
                    let path = file.path();
                    let data = fs::read_to_string(path).expect("Unable to read file");
                    // parse graphql query to ast
                    let ast = parse_query::<&str>(data.as_str()).expect("failed to parse the file");

                    // transform ast to json
                    let query: Query = from_value(json!({
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
                    }))?;
                    //println!("{:#?}", q);

                    //todo: this needs to be broken into a module
                    // write to file
                    let file = File::create("collection.json").expect("couldn't create file");
                    to_writer_pretty(file, &query).expect("failed to write to file");
                }
            }
            None => ()
        }
    }
    Ok(())
}
