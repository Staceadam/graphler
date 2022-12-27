use crate::collection::{Collection, Query};
use apollo_parser::{
    ast::{self, AstChildren, NamedType, OperationDefinition, Selection, SelectionSet},
    Parser, TokenKind,
};
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::Path;
use walkdir::WalkDir;

fn get_selection(set: ast::SelectionSet) {
    for selection in set.selections() {
        match selection {
            Selection::FragmentSpread(frag) => {
                println!("fragment spread: {:#?}", frag)
            }
            Selection::InlineFragment(frag) => {
                println!("{:#?}", frag)
            }
            Selection::Field(field) => {
                if let Some(selection_set) = field.selection_set() {
                    get_selection(selection_set);
                }
            }
        }
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
                                // if there is a fragment add it to a hashmap/list that persists through each files lifetime
                                // this might have to instead do an initial pass through all the files and only grab the fragments
                                // then go through the definitions again without taking fragment definitions into consideration
                                println!("{:?}", frag_def);
                            }
                            ast::Definition::OperationDefinition(op_def) => {
                                let name = op_def.name().unwrap().text().as_str().to_string();

                                if let Some(selection_set) = op_def.selection_set() {
                                    get_selection(selection_set);
                                }

                                // let selection =
                                //     selection_set.iter().map(|x| x.selections()).flatten();

                                // let flat_two = selection_set.iter()

                                // check if operation has any fragments in it
                                // if so grab all fragments in operation and then check them against list of fragments to find a match
                                // prepend entire fragment definition onto the data string that gets passed into query
                                // println!("{:#?}", op_def.selection_set().unwrap().selections().any(f));
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
                                        ast::Type::ListType(list_type) => {
                                            //TODO: need to dig into how to resolve a list, that is something that comes in as [String] etc...
                                            /*
                                                "variables":{"ids":["0"]
                                            */
                                            variables.insert(name, Some("idk".to_string()));
                                        }
                                        ast::Type::NamedType(named) => {
                                            let str_type = named
                                                .name()
                                                .unwrap()
                                                .ident_token()
                                                .unwrap()
                                                .text()
                                                .to_string();

                                            if str_type == "ID" {
                                                variables.insert(name, None);
                                            } else if str_type == "String" {
                                                variables.insert(name, None);
                                            } else if str_type == "Boolean" {
                                                variables.insert(name, None);
                                            } else if str_type == "Float" {
                                                variables.insert(name, None);
                                            } else if str_type == "Int" {
                                                variables.insert(name, None);
                                            } else {
                                                //TODO: theres no real way to check what a unique types shape is
                                                variables.insert(name, None);
                                            }
                                        }
                                        ast::Type::NonNullType(non_null) => {
                                            let str_type = non_null
                                                .named_type()
                                                .unwrap()
                                                .name()
                                                .unwrap()
                                                .ident_token()
                                                .unwrap()
                                                .text()
                                                .to_string();

                                            if str_type == "ID" {
                                                variables.insert(name, Some("0".to_string()));
                                            } else if str_type == "String" {
                                                variables.insert(name, Some("test".to_string()));
                                            } else if str_type == "Boolean" {
                                                variables.insert(name, Some("true".to_string()));
                                            } else if str_type == "Float" {
                                                variables.insert(name, Some("1.2".to_string()));
                                            } else if str_type == "Int" {
                                                variables.insert(name, Some("1".to_string()));
                                            } else {
                                                //TODO: theres no real way to check what a unique types shape is
                                                variables.insert(name, Some("{}".to_string()));
                                            }
                                        }
                                    }
                                }
                                let query = Query::new(name, &data, variables);

                                collection.item.push(query)
                            }
                            _ => println!("not found"),
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
    println!("{:#?}", parsed)
}
