extern crate postgres;

use postgres::{Client, NoTls};

fn main() {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    client.batch_execute(
        "
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
        ",
    )?;

    let name = "Ferris";
    let data = None::<&[u8]>;

    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    );

    for row in client.query("SELECT id, name, data FROM person", &[])? {
        let id = row.get(0);
        let name = row.get(1);
        let data = row.get(2);
        println!("found person: {} {} {:?}", id, name, data);
    }
}
