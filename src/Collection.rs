use graphql_parser::query;
use serde::{Deserialize, Serialize};

use serde::de::{Deserialize as De, Deserializer, Error, Unexpected};

#[derive(Serialize, Deserialize, Debug)]
struct Url {
    raw: String,
    protocol: String,
    host: Vec<String>,
    path: Vec<String>,
}
#[derive(Serialize, Debug)]
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

impl<'de> De<'de> for Graphql {
    fn deserialize<D>(deserializer: D) -> Result<Graphql, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        println!("{}", s);
        //s.parse()
        //.map(Value)
        //.map_err(|_| D::Error::invalid_value(Unexpected::Str(s), &"a floating point number as a string"))
    }
}

//pub fn build_collection_query<'b>(ast: &'b query::Document<&'b str>) -> Query {
//Query::new()

//let query: Query = from_value(json!({
//"name": "character",
//"request": {
//"method": "POST",
//"header": [],
//"body": {
//"mode": "graphql",
//"graphql": {
//"query": format!("{}", &ast),
//"variables": "{\n\t\"id\": \"0\"\n}"
//}
//},
//"url": {
//"raw": "https://rickandmortyapi.com/graphql",
//"protocol": "https",
//"host": [
//"rickandmortyapi",
//"com"
//],
//"path": [
//"graphql"
//]
//}
//},
//"response": []
//}))
//.expect("Couldn't read the ast that was passed");
//query
//}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    name: String,
    request: Request,
    response: Vec<String>,
}

impl Query {
    pub fn new<'a>(ast: &'a query::Document<&'a str>) -> Query {
        println!("{:#?}", ast);
        Query {
            name: "character".to_owned(),
            request: Request {
                method: "POST".to_owned(),
                header: vec![],
                body: Body {
                    mode: "graphql".to_owned(),
                    graphql: Graphql {
                        query: format!("{}", ast),
                        variables: "{\n\t\"id\": \"0\"\n}".to_owned(),
                    },
                },
                url: Url {
                    raw: "https://rickandmortyapi.com/graphql".to_owned(),
                    protocol: "https".to_owned(),
                    host: vec!["rickandmortyapi".to_owned(), "com".to_owned()],
                    path: vec!["graphql".to_owned()],
                },
            },
            response: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub schema: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    info: Info,
    pub item: Vec<Query>,
}

impl Collection {
    pub fn new(name: &str) -> Collection {
        Collection {
            info: Info {
                name: name.to_owned(),
                schema: "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
                    .to_owned(),
            },
            item: Vec::new(),
        }
    }
}
