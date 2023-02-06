use clap::Parser;
use rsmgclient::{ConnectParams, Connection, SSLMode};

#[derive(Parser)]
struct Cli {
    filename: String,
}

fn main() -> Result<(), String> {
    markdown_fn::debug();

    let args = Cli::parse();
    println!("filename: {}", args.filename);

    // Parameters for connecting to database.
    let connect_params = ConnectParams {
        host: Some(String::from("localhost")),
        sslmode: SSLMode::Disable,
        ..Default::default()
    };

    // Make a connection to the database.
    let mut connection = match Connection::connect(&connect_params) {
        Ok(c) => c,
        Err(err) => panic!("{}", err),
    };

    // Execute a query.
    let query =
        "CREATE (u:User {name: 'Alice'})-[:Likes]->(m:Software {name: 'Memgraph'}) RETURN u, m";
    match connection.execute(query, None) {
        Ok(columns) => println!("Columns: {}", columns.join(", ")),
        Err(err) => panic!("{}", err),
    };

    // Fetch all query results.
    match connection.fetchall() {
        Ok(records) => {
            for value in &records[0].values {
                println!("{}", value);
            }
        }
        Err(err) => panic!("{}", err),
    };

    // Commit any pending transaction to the database.
    match connection.commit() {
        Ok(()) => Ok(()),
        Err(err) => panic!("{}", err),
    }
}
