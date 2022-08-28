# Graphler

- cli
- build in rust
- outputs to a .graphler directory at root

## Purpose
create a solution for managing a graphql postman collection that involves
1. iterating over local or remote repo directories to find any .graphql/.gql files
2. using the .graphql.config.yml to provide headers needed to interact with endpoint
3. creating a collection.json with correct formatting for a postman collection
4. looks to see if there is an existing collection.json and uses any existing variables with query matches

checkout https://deno.land/x/graphman@v1.0.0?source for inspiration

```json
{
	"info": {
		"name": "rickandmortyapi.com-GraphMan",
		"schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
	},
	"item": [
		{
			"name": "character",
			"request": {
				"method": "POST",
				"header": [],
				"body": {
					"mode": "graphql",
					"graphql": {
						"query": "query character($id: ID!) {\n  character(id: $id) {\n    __typename\n    id # The id of the character.\n    name # The name of the character.\n    status # The status of the character ('Alive', 'Dead' or 'unknown').\n    species # The species of the character.\n    type # The type or subspecies of the character.\n    gender # The gender of the character ('Female', 'Male', 'Genderless' or 'unknown').\n    # origin # The character's origin location\n    # location # The character's last known location\n    image # Link to the character's image. All images are 300x300px and most are medium shots or portraits since they are intended to be used as avatars.\n    # episode # Episodes in which this character appeared.\n    created # Time at which the character was created in the database.\n  }\n}",
						"variables": "{\n\t\"id\": \"0\"\n}"
					}
				},
				"url": {
					"raw": "https://rickandmortyapi.com/graphql",
					"protocol": "https",
					"host": [
						"rickandmortyapi",
						"com"
					],
					"path": [
						"graphql"
					]
				}
			},
			"response": []
		},

```
