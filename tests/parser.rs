use serde_json::json;
use graphler::parser::parse;
use graphler::collection::{Collection, Query};
use serde_test::{Token, assert_tokens};

	//"info": {
		//"name": "rickandmortyapi.com-GraphMan",
		//"schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
	//},
	//"item": [
		//{
			//"name": "character",
			//"request": {
				//"method": "POST",
				//"header": [],
				//"body": {
					//"mode": "graphql",
					//"graphql": {
						//"query": "query character($id: ID!) {\n  character(id: $id) {\n    __typename\n    id # The id of the character.\n    name # The name of the character.\n    status # The status of the character ('Alive', 'Dead' or 'unknown').\n    species # The species of the character.\n    type # The type or subspecies of the character.\n    gender # The gender of the character ('Female', 'Male', 'Genderless' or 'unknown').\n    # origin # The character's origin location\n    # location # The character's last known location\n    image # Link to the character's image. All images are 300x300px and most are medium shots or portraits since they are intended to be used as avatars.\n    # episode # Episodes in which this character appeared.\n    created # Time at which the character was created in the database.\n  }\n}",
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
		//},
        //


       //let json_value: Value = from_str(s).unwrap();
        //assert_eq!(json_value, to_value(&value).unwrap());

        //// Make sure we can deserialize from a `&Value`.
        //let v = T::deserialize(&json_value).unwrap();
        //assert_eq!(v, value);

        //// Make sure we can deserialize from a `Value`.
        //let v: T = from_value(json_value.clone()).unwrap();
        //assert_eq!(v, value);

        //// Make sure we can round trip back to `Value`.
        //let json_value2: Value = from_value(json_value.clone()).unwrap();
        //assert_eq!(json_value2, json_value);
        //

/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct S {
///     a: u8,
///     b: u8,
/// }
///
/// let s = S { a: 0, b: 0 };
/// assert_tokens(&s, &[
///     Token::Struct { name: "S", len: 2 },
///     Token::Str("a"),
///     Token::U8(0),
///     Token::Str("b"),
///     Token::U8(0),
///     Token::StructEnd,
/// ]);

//fn build_mock_collection() -> Collection {
    //Collection::new("testColleection")
//}
    //let mock_collection = build_mock_collection();
    //println!("{:#?}", mock_collection);

#[test]
fn test_info() {
    let parsed = parse("src/etc").expect("parsing failed");

    assert_tokens(&parsed, &[
        Token::Map { len: Some(0) },
        Token::MapEnd,
    ]);
}

#[test]
fn test_multiple_items() {
    // todo: break up parser into parseing and file creation
    // test parser output against a mocked valid postman.collection.json
}

#[test]
fn test_item_name() {
    // todo: break up parser into parseing and file creation
    // test parser output against a mocked valid postman.collection.json
}
