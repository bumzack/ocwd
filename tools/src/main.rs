use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::iter::Map;
use tokio_postgres::types::Type;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tokio_postgres().await.expect("tokio_postgres() error");

    println!("finished");
    Ok(())
}

pub type DbRowAsJson = Map<String, Value>;

async fn tokio_postgres() -> Result<(), Box<dyn Error>> {
    println!("get client");
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=ollamachat password=ollamachat dbname=ollamachat",
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query("SELECT * FROM ollama_model", &[]).await?;

    println!("query database");

    //  let params = rows[0].try_get::<&str, serde_json::Value>("params")?;
    // let rows = client
    //     .query("select * from ollama_model", &[])
    //     .await
    //     .expect("query");

    println!("{:?}", rows);
    // let rows= match rows {
    //      Ok(rows) => {
    //          println!("rows: {:?}", rows);
    //          rows
    //      },
    //      Err(err) => {
    //          println!("error err {:?}",err);
    //          return Err(Box::new(err));
    //      }
    //  };

    println!("iterate");

    // let people: Vec<HashMap<String, String>> = serde_postgres::from_rows(&r)?;
    for row in rows {
        let mut map = HashMap::new();
        for (idx, column) in row.columns().into_iter().enumerate() {
            let value = match *column.type_() {
                Type::INT4 => row.get::<_, i32>(idx).to_string(),
                Type::INT8 => row.get::<_, i64>(idx).to_string(),
                Type::TEXT | Type::VARCHAR => row.get::<_, String>(idx),
                Type::BOOL => row.get::<_, bool>(idx).to_string(),
                Type::TIMESTAMPTZ => row.get::<usize, DateTime<Utc>>(idx).to_string(),

                _ => panic!("todo add impl"),
            };
            map.insert(column.name().to_string(), value);
        }
        println!("{:?}", map);
    }

    Ok(())
}
