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

For inspiration checkout: [graphman](https://deno.land/x/graphman@v1.0.0)

## V1
[] smart variables that grab off of query
[] visitor solution
[] directory at $HOME
	- base config
	- stores output
	- supports one project
[] cli
	- request for headers

## V2
[] checks existing collection.json against input of .graphql files
[] base directory supports multiple "projects"
[] cli
	- supports setting a current project
	- supports multiple projects
	- has add a new project
[] js support
[] insomnia plugin
