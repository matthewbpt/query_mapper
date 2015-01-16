extern crate postgres;
extern crate time;

#[macro_use]
extern crate query_mapper;

use std::fmt;
use time::Timespec;

use postgres::{Connection, SslMode};

struct Person {
    id: i32,
    name: String
}

impl fmt::Show for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person {{ id: {}, name: {} }}", self.id, self.name)
    }
}

fn main() {    
/*
    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    time_created    TIMESTAMP NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Matthew".to_string(),
        time_created: time::get_time(),
        data: None
    };

    let you = Person {
        id: 1,
        name: "Yannick".to_string(),
        time_created: time::get_time(),
        data: None
    };

    conn.execute("INSERT INTO person (name, time_created, data)
                    VALUES ($1, $2, $3)",
                 &[&you.name, &you.time_created, &you.data]).unwrap();
*/
    let conn_uri = "postgres://postgres:password@localhost:5555";
    let conn = match Connection::connect(conn_uri, &SslMode::None) { Ok(c) => c, Err(m) => return };

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
}
