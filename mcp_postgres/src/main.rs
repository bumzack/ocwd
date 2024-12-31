use tokio_postgres::{Error, NoTls};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=ollamachat password=ollamachat", NoTls)
            .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client.query("SELECT * FROM ollama_chat", &[]).await?;

    // And then check that we got back the same string we sent over.

    let mut res = vec![];

    for row in rows {
        let mut r = vec![];
        for (idx, col) in row.columns().iter().enumerate() {
            println!("{}: {}", col.name(), col.type_());
            println!("{}", row.get(idx));
            // let x = format!("\"{}\": \"{:?}\"", col.name(), row.get(idx));
            // r.push(x);
        }
        res.push(r);
    }

    println!("{:?}", res);

    Ok(())
}
