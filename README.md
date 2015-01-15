# query_mapper for Rust

This work in progress library allows you to run SQL queries against a postgres database and have the result mapped to a vector of a struct automatically. 
The query_map! macro handles mapping between columns in the query and fields in the struct.

It is inspired by the Dapper micro ORM for .Net

## Usage

```rust
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
            
// Then use query_map! macro to map a result set to a vec of "Person"s
for person in query_map!(conn, preparedSqlString, &[&1],
	Person { id, name, time_created, data } // The struct name and fields within the struct
	).iter() {
	println!("id: {}, name: {}", person.id, person.name)
}

// You can also specify custom mapping of columns to fields
for person in query_map!(conn, preparedSqlString, &[&1], 
	Person { id => "id", name => "name", time_created => "time_created", data => "data" }
	).iter() {
	println!("id: {}, name: {}", person.id, person.name)
}


```
