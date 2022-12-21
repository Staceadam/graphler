use std::fs;
use std::io::Error;
use std::path::Path;
use apollo_parser::{ast::{self, OperationDefinition, NamedType}, Parser, TokenKind};
use walkdir::WalkDir;
use crate::collection::{Collection, Query};
use std::collections::HashMap;


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
                                let name = op_def.name().unwrap().text().as_str().to_string();
                                
						// "variables": "{\n\t\"page\": null,\n\t\"filter\": null\n}"
                        //vs
                        // graphql: Graphql {
                        //     variables: "id name",
                        // },

                                // variables
                                let variable_defs = op_def.variable_definitions();
                                let flat = variable_defs
                                    .iter()
                                    .map(|v| v.variable_definitions())
                                    .flatten();
                                
                                //TODO: these need to check an existing collection before setting them to dummy values
                                let mut variables = HashMap::new(); 

                                for var_def in flat {
                                    let name = var_def.variable().unwrap().text().to_string();
                                    match var_def.ty().unwrap() {
                                       ast::Type::ListType(test) => {
                                            variables.insert("list".to_string(), "idk".to_string());
                                       },
                                       ast::Type::NamedType(named) => {
                                            let str_type = named.name().unwrap().ident_token().unwrap().text().to_string();
                                            // type is nullable, set it to null
                                            // TODO: need to set it as null not "null"
                                            if str_type == "ID" {
                                                variables.insert(name, "".to_string());
                                            } else if str_type == "String" {
                                                variables.insert(name, "".to_string());
                                            } else if str_type == "Boolean" {
                                                variables.insert(name, "".to_string());
                                            } else if str_type == "Float" {
                                                variables.insert(name, "".to_string());
                                            } else if str_type == "Int" {
                                                variables.insert(name, "".to_string());
                                            } else {
                                                //TODO: check other types and create object or list
                                                variables.insert(name, "null".to_string());
                                            }
                                            // buf.push_str(&var_def.variable().unwrap().text().to_string());
                                            // buf.push_str(&named.name().unwrap().ident_token().unwrap().to_string());
                                        // println!("{:#?}", named.name().unwrap().ident_token().unwrap().to_string())
                                       },
                                       ast::Type::NonNullType(non_null) => {
                                        let str_type = non_null.named_type().unwrap().name().unwrap().ident_token().unwrap().text().to_string();

                                        if str_type == "ID" {
                                            variables.insert(name, "0".to_string());
                                        } else if str_type == "String" {
                                            variables.insert(name, "test".to_string());
                                        } else if str_type == "Boolean" {
                                            variables.insert(name, "false".to_string());
                                        } else if str_type == "Float" {
                                            variables.insert(name, "1.2".to_string());
                                        } else if str_type == "Int" {
                                            variables.insert(name, "1".to_string());
                                        } else {
                                            //TODO: check other types and create object or list
                                            variables.insert(name, "unique".to_string());
                                        }
                                       }
                                    }
                                }
                                let query = Query::new(name, &data, variables);

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