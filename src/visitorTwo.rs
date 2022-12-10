use graphql_parser::{query::{
    Definition, DirectiveDefinition, Document, EnumType, EnumValue, Field, InputObjectType,
    InputValue, InterfaceType, ObjectType, ScalarType, SchemaDefinition, TypeDefinition, UnionType,
    OperationDefinition
}, schema::Text};

pub trait OperationVisitor {
    fn visit_schema_document<'a>(&self, document: &Document<'a, &'a str>) {
        self.visit_document();

        for definition in document.definitions {
            let query_type = None;
            match definition {
                Definition::SchemaDefinition(schema_definition) => {
                    self.visit_schema_definition(schema_definition);
                    query_type = schema_definition.query_type().name
                }
                // Definition::Fragment(fragment) => {
                //     let TypeCondition::On(name) = &fragment.type_condition;
                //     Some(name)
                // }
                Definition::Operation(operation) => match operation {
                    OperationDefinition::Query(_) => Some(query_type),
                    // OperationDefinition::SelectionSet(_) => Some(&context.schema.query_type().name),
                    // OperationDefinition::Mutation(_) => context.schema.mutation_type().map(|t| &t.name),
                    // OperationDefinition::Subscription(_) => {
                    //     context.schema.subscription_type().map(|t| &t.name)
                    // }
                },
            }
        }
    }

    fn visit_document(&self) {}
    fn visit_schema_definition(&self) {}
}


#[test]
fn test_visitor() {
    use graphql_parser::parse_query;

    struct TestVisitor;
    impl<'a> TestVisitor {
        fn collect_visited_info(&self, document: &Document<'a, &'a str>) {
            self.visit_schema_document(document);
            println!("working");

            //todo: collect
            // collected
        }
    }

    impl OperationVisitor for TestVisitor  {
        fn visit_document(&self) {

        }
    }

    let doc = parse_query::<&str>(r#"
        query TestQuery(input) {
            users {
                id
                country {
                    id
                }
            }
        }
    "#).expect("Failed to parse query");

    let visitor = TestVisitor {};
    let collected = visitor.collect_visited_info(&doc);

    // assert_eq!(
    //     collected.collected_object_type,
    //     vec!["Query", "User", "Test"]
    // );

    // let mut fields = 0;
    // let mut field_names = Vec::new();
    // for f in doc.visit() {
    //     fields += 1;
    //     field_names.push(f.name);
    // }
    // assert_eq!(fields, 4);
    // assert_eq!(field_names, vec!["users", "id", "country", "id"]);
}