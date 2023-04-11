# rust_contact_list

TODO: ADD Interactivity

A small application demonstrating basic functionality of a Contact List service.
Features:
* serializing/deserializing contact lists from/to json files
* search contacts
* create/edit/delete
* pagination

The server starts on port 8080. 

Command examples:

Get ALL contacts:
curl  http://localhost:8080/contacts/all

Get paginated contacts with the given offset:
curl http://localhost:8080/contacts?offset=3

ADD contact:
curl -d '{"name" : "Illia", "phone" : "1233444455", "email" : "illia@company.com"}' -H "Content-Type: application/json" -X POST http://localhost:8080/contacts

EDIT contact:
curl -d '{"name" : "Illia", "phone" : "1233444455", "email" : "illia@company.com"}' -H "Content-Type: application/json" -X PUT http://localhost:8080/contacts

DELETE contact:
curl -X DELETE http://localhost:8080/contacts/Alice


Implementation details (main data structures)
* Internally Contact List application is using BTreeMap for faster pagination, inserts, deletes and edits. Additionally, it uses Tries for prefix-based lookups. 





Usage:
To run the functionality demo, build the application with `cargo build` and run the resulting binary providing 2 arguments: 1st argument is path to the initial contact list dataset in json format (see samples dir for examples of valid datasets) and 2nd argument is path to the resulting output file (will be produced by the program at the end of the demo run).

See unit tests for more examples.

Demo run example:

```
cargo build & ./target/debug/contact_list ./samples/example.json ./samples/example_output.json
```
