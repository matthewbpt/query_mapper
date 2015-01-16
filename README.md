# query_mapper for Rust [![Build Status](https://travis-ci.org/matthewbpt/query_mapper.svg?branch=master)](https://travis-ci.org/matthewbpt/query_mapper)

This work in progress library allows you to run SQL queries against a postgres database and have the result mapped to a vector of a struct automatically. 
The query_map! macro handles mapping between columns in the query and fields in the struct.

It is inspired by the Dapper micro ORM for .Net

## Usage

```rust
// Imagine this struct

struct Person {
   id: i32,
   name: String
}

// Create a database connection
let conn_uri = "postgres://postgres:password@localhost";
let conn = match Connection::connect(conn_uri, &SslMode::None) { Ok(c) => c, Err(m) => return () };

// prepare a string
let prepared_sql_string = "SELECT id, name FROM person WHERE id = $1";

// use the query_map! macro to execute the query and map each row to a Person
// with columns "id" mapped to id and "name" mapped to name, it takes the
// names of the fields specified and maps the column with the same name
let result = query_map!(conn, prepared_sql_string, &[&2],
    Person { id, name }
);

// iterate through resultset and print each row
match result {
    Ok(r) => {
            for person in r.iter() {
                println!("id: {}, name: {}", person.id, person.name);
        }
    },
    Err(m) => println!("{:?}", m),
};

let another_prepared_sql_string = "SELECT person_id, full_name FROM person WHERE id = $1";

// same as above, but instead we provide a custom mapping if the columns
// in the query don't match fields in the struct
let result2 = query_map!(conn, another_prepared_sql_string, &[&1], 
    Person { id => "person_id", name => "full_name" }
);

match result2 {
    Ok(r) => {
        for person in r.iter() {
            println!("id: {}, name: {}", person.id, person.name);
        }
    },
    Err(m) => println!("{:?}", m),
};

```
