# query_mapper for Rust

This work in progress library allows you to run SQL queries against a postgres database and have the result mapped to a vector of a struct automatically. 
The map_query! macro handles mapping between columns in the query and fields in the struct.

It is inspired by the Dapper micro ORM for .Net

## Usage

```
// Imagine this struct

struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>
}

// First create a connection
let conn_uri = "postgres://postgres:skid555@localhost:5555";
	
let conn = Connection::connect(conn_uri, &SslMode::None)
            .unwrap();
            
// Then use map_query! macro to map a result set to a vec of "Person"s

for person in query_map!(conn, "SELECT id, name, time_created, data FROM person WHERE id = $1", 
	&[&1], // The parameter to pass the the prepared statement
	Person, id, name, time_created, data // The struct name and fields within the struct
	)
	.iter() {
		println!("id: {}, name: {}", person.id, person.name)
	}

```