use std::fs;
use std::io::Error;
use std::path::Path;
use apollo_parser::{ast::{self, OperationDefinition}, Parser};
use walkdir::WalkDir;

//this is dumb
use crate::collection::{Collection, Query};

fn operation_type_text(op_def: &OperationDefinition) -> String {
    if op_def.operation_type().unwrap().query_token().is_some() {
        return "query".to_string()
    } else if op_def.operation_type().unwrap().mutation_token().is_some() {
        return "mutation".to_string()
    } else {
        return "subscription".to_string()
    }
}

pub fn parse(path: &str) -> Result<Collection, Error> {
    let mut collection = Collection::new("insertNameFromCliInputOrUrlBase");
    //todo: could do the extension match in the filter_map here
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        let extension = Path::new(file.file_name()).extension();
        match extension {
            Some(val) => {
                let ext = val
                    .to_str()
                    .expect("failed to get file with that extension");
                if ext == "graphql" || ext == "gql" {
                    let data = fs::read_to_string(file.path()).expect("Unable to read file");
                    let parser = Parser::new(&data);
                    let ast = parser.parse();

                    let doc = ast.document();

                    for def in doc.definitions() {
                        match def {
                            ast::Definition::FragmentDefinition(frag_def) => {
                                println!("{:?}", frag_def);
                            },
                            ast::Definition::OperationDefinition(op_def) => {
                                // needs to check operation type for query/mutation/subscription
                                let op_type = operation_type_text(&op_def);
                                let name = op_def.name().unwrap().text().as_str().to_string();
                                let query = format!("{} {}", op_type, name);

                                // variables
                                let variable_defs = op_def.variable_definitions();
                                let variables: Vec<String> = variable_defs
                                    .iter()
                                    .map(|v| v.variable_definitions())
                                    .flatten()
                                    .filter_map(|v| Some(v.variable()?.text().to_string()))
                                    .collect();

                                println!("variables: {:?}, string: {}", variables, variables.join(" "));
                                let query = Query::new(name, query, variables.join(" "));
                                
                                collection.item.push(query)
                            },
                            _ => println!("not found") 
                        }
                    }
                }
            }
            None => println!("Couldn't find any .graphql or .gql files in this project"),
        }
    }
    Ok(collection)
}


#[test]
fn test_info() {
    let parsed = parse("etc").expect("parsing failed");
}