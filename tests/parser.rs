use graphler::parser::parse;
use serde_json::{json};
use graphler::collection;

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

#[test]
fn test_colletion() {
    //todo: find way to make blanket assertion against collection
    parse("src/etc/single").expect("parsing failed");
    //assert_eq!(col, true);
}

#[test]
fn test_info() {
    let col = json!(parse("src/etc/single").expect("parsing failed"));
    //todo: make this more generic
    let info = collection::Info {
        name: String::from("insertNameFromCliInputOrUrlBase"),
        schema: String::from("https://schema.getpostman.com/json/collection/v2.1.0/collection.json"),
    };
    assert_eq!(*col.get("info").unwrap(), json!(info));
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
