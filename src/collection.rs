use graphql_parser::query;
//use serde::de::{Deserialize as De, Deserializer, Error, Unexpected};
use graphql_tools::ast::schema_visitor::SchemaVisitor;
use graphql_tools::static_graphql::schema:: {
    Definition, Document, EnumType, EnumValue, Field, InputObjectType,
    InputValue, InterfaceType, ObjectType, ScalarType, SchemaDefinition, TypeDefinition, UnionType,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

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

struct QueryVisitor;

impl QueryVisitor {
    fn create_query(&self, document: &Document) -> Graphql {
        let mut graphql = Graphql {
            query: String::new(),
            variables: String::new()
        };
        self.visit_schema_document(document, &mut graphql);

        graphql
    }
}

impl SchemaVisitor<Graphql> for QueryVisitor {
    fn enter_object_type(
        &self,
        _node: &ObjectType,
        _visitor_context: &mut Graphql,
    ) {
        // _visitor_context
        //     .query
        println!("{}", _node.name);
    }

    fn enter_object_type_field(
        &self,
        _node: &Field,
        _type_: &ObjectType,
        _visitor_context: &mut Graphql,
    ) {
        // let field_id = format!("{}.{}", _type_.name.as_str(), _node.name.as_str());
        // _visitor_context.query
        println!("{}", _type_.name);
        println!("{}", _node.name);
    }

    fn enter_input_object_type(
        &self,
        _node: &InputObjectType,
        _visitor_context: &mut Graphql,
    ) {
        println!("{}", _node.name);
        // _visitor_context
        //     .query
    }

    fn enter_input_object_type_field(
        &self,
        _node: &InputValue,
        _input_type: &InputObjectType,
        _visitor_context: &mut Graphql,
    ) {
        // let field_id = format!("{}.{}", _input_type.name.as_str(), _node.name.as_str());
        // _visitor_context.query;
        println!("{}", _input_type.name);
        println!("{}", _node.name);
    }
}

impl Query {
    pub fn new<'ast>(ast: &'ast query::Document<&'ast str>) -> Query {
    
        let visitor = QueryVisitor {};
        let graphql = visitor.create_query(&ast);

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
