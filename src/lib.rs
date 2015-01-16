#![crate_name = "query_mapper"]
mod query_mapper {
/// A macro that will execute a given SQL query and return a Vector $T of a given
/// struct (wrapped in a Result<$T>) with columns in the result set mapped to
/// fields in the struct.
///
/// The macro comes in two forms: 
/// 
/// The first one takes a struct ($T) and a list of
/// fields ($member), and it will use the field names to map to columns  in the result
/// set with the same name.
///
/// The second one is like the first, but allows you to specify custom mapping between
/// each field and column in the form $member => $key , with $member being the field name
/// in the struct and $key being the column name in the result set
///
/// # Example
///
/// ``` 
/// # #[macro_use] extern crate query_mapper;
/// # extern crate postgres;
/// use postgres::{Connection, SslMode};
///
/// struct Person {
///    id: i32,
///    name: String
/// }
/// # fn main() {
/// # let conn_uri = "postgres://postgres:password@localhost";
/// 
/// let conn = match Connection::connect(conn_uri, &SslMode::None) { Ok(c) => c, Err(m) => return () };
/// 
/// let prepared_sql_string = "SELECT id, name FROM person";
///
/// let result = query_map!(conn, prepared_sql_string, &[],
///     Person { id, name }
/// );
///
/// match result {
///     Ok(r) => {
///         for person in r.iter() {
///             println!("id: {}, name: {}", person.id, person.name);
///     }
/// },
///     Err(m) => println!("{:?}", m),
/// };
///
/// let result2 = query_map!(conn, prepared_sql_string, &[], 
///     Person { id => "id", name => "name" }
/// );
///
/// match result2 {
///     Ok(r) => {
///         for person in r.iter() {
///             println!("id: {}, name: {}", person.id, person.name);
///         }
///     },
///     Err(m) => println!("{:?}", m),
/// };
/// 
/// # }
/// ```
#[macro_export]
macro_rules! query_map {
    // Macro  will auto map struct $T with members $member, asuming that
    // the columns returned by the query match the field names in the struct. 
    // Must pass in all fields in the struct as idents
    ($conn:ident, $query:expr, $params:expr, $T:ident{ $($member:ident),* }) => {
        {
            match $conn.prepare($query) {
                Ok(statement) => {
                    match statement.query($params) {
                        Ok(result) => { 
                            Ok(result.map(|row:postgres::Row| { 
                                $T { $( $member: row.get(stringify!($member)) ),*} 
                            }).collect::<Vec<$T>>()) 
                        },
                        Err(message) => Err(message)
                    }
                },
                Err(message) => Err(message)            
            }
        }
    };
    // Similar to above, but each field is mapped to an expression which should be
    // a string. This allows you specify custom column mapping when column
    // names don't match field names
    ($conn:ident, $query:expr, $params:expr, $T:ident{ $($member:ident => $key:expr),* }) =>{
        {
            match $conn.prepare($query) {
                Ok(statement) => {
                    match statement.query($params) {
                        Ok(result) => {
                            Ok(result.map(|row:postgres::Row| {
                                $T { $( $member: row.get($key) ),* }
                            }).collect::<Vec<$T>>())
                        },
                        Err(message) => Err(message)
                    }
                },
                Err(message) => Err(message)
            }           
        }
    }
}
}
