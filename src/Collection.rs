use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
    variables: HashMap<String, Option<String>>,
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

impl Query {
    pub fn new(name: String, query: &str, variables: HashMap<String, Option<String>>) -> Self {
        Query {
            name,
            request: Request {
                method: "POST".to_owned(),
                header: Vec::new(),
                body: Body {
                    mode: "graphql".to_owned(),
                    graphql: Graphql {
                        query: query.to_string(),
                        variables
                    }
                },
                url: Url {
                    raw: "https://rickandmortyapi.com/graphql".to_owned(),
                    protocol: "https".to_owned(),
                    host: vec!["rickandmortyapi".to_owned(), "com".to_owned()],
                    path: vec!["graphql".to_owned()]
                }
            },
            response: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    name: String,
    schema: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    info: Info,
    pub item: Vec<Query>
}

impl Collection {
    pub fn new(name: &str) -> Collection {
        Collection {
            info: Info {
                name: name.to_owned(),
                schema: "https://schema.getpostman.com/json/collection/v2.1.0/collection.json".to_owned()
            },
            item: Vec::new()
        }
    }
}

