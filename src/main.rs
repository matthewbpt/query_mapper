extern crate postgres;
extern crate time;

#[macro_use]
extern crate query_mapper;

use time::Timespec;

use postgres::{Connection, SslMode};

struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>
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
	
	let conn = Connection::connect(conn_uri, &SslMode::None)
            .unwrap();
	
	for person in query_map!(conn, "SELECT id, name, time_created, data FROM person WHERE id = $1", &[&2], Person, id, name, time_created, data).iter() {
		println!("id: {}, name: {}", person.id, person.name)
	}	
}
