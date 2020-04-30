extern crate postgres;

use postgres::{Client, NoTls};
#[derive(Debug)]
struct Person{
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

#[allow(dead_code)]
fn main() {
    let mut client = match Client::connect("host=localhost user=postgres", NoTls) {
        Ok(client) => client,
        Err(e) => {
            println!("hell {}", e);
            return
        }
    };

    client.batch_execute("
        CREATE TABLE person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
    ").expect("Table creation failed");

    let me = Person{
        id: 0,
        name: "Steven".to_owned(),
        data: None
    };

    client.execute("INSERT INTO person (name, data) VALUES ($1, $2)", &[&me.name, &me.data]).expect("INSERT failed");

    for row in &client.query("SELETE id, name, data FROM person", &[]).unwrap(){
        let person = Person{
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person {:?}", person);
    }
}
