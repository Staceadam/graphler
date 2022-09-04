
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

