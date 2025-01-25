use chrono::{DateTime, Utc};
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::error::Error;
use tokio_postgres::types::Type;
use tokio_postgres::NoTls;

pub struct PostgresUtilRequest {
    pub query: String,
}

pub struct PostgreSqlConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub db_name: String,
}

// based on this https://github.com/sfackler/rust-postgres/issues/284#issuecomment-1092443247
pub async fn tokio_postgres(
    req: &PostgresUtilRequest,
    config: PostgreSqlConfig,
) -> Result<Vec<HashMap<String, Value>>, Box<dyn Error>> {
    let conf_str = format!(
        "host={} user={} password={} dbname={}",
        config.host, config.username, config.password, config.db_name
    );
    let (client, connection) = tokio_postgres::connect(conf_str.as_str(), NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let lower = req.query.clone();
    let lower = lower.as_str().to_ascii_lowercase();

    // poor mans safety check
    if lower.contains("insert") || lower.contains("delete") || lower.contains("update") {
        return *Box::new(Err(
            "you are not allowed to update, insert or delete. only select statements are valid"
                .into(),
        ));
    }

    let rows = client.query(&req.query, &[]).await?;

    let mut row_as_values: Vec<HashMap<String, Value>> = Vec::new();
    for row in rows {
        let mut map = HashMap::new();
        for (idx, column) in row.columns().iter().enumerate() {
            let value = match *column.type_() {
                Type::INT4 => Value::Number(Number::from(row.get::<_, i32>(idx))),
                Type::INT8 => Value::Number(Number::from(row.get::<_, i64>(idx))),
                Type::FLOAT4 => Value::Number(
                    Number::from_f64(row.get::<_, f32>(idx) as f64).expect("should be a number"),
                ),
                Type::FLOAT8 => Value::Number(
                    Number::from_f64(row.get::<_, f64>(idx)).expect("should be a number"),
                ),
                Type::TEXT | Type::VARCHAR => Value::String(row.get::<_, String>(idx)),
                Type::BOOL => Value::Bool(row.get::<_, bool>(idx)),
                Type::TIMESTAMPTZ => {
                    Value::String(row.get::<usize, DateTime<Utc>>(idx).to_string())
                }
                _ => {
                    println!("error converting type {} to a JSON value", *column.type_());
                    panic!(
                        "can't convert type {} to serde value. add impl for this type",
                        *column.type_()
                    );
                }
            };
            map.insert(column.name().to_string(), value);
        }
        let v = serde_json::to_string_pretty(&map).unwrap();
        row_as_values.push(map);
    }

    let v = serde_json::to_string_pretty(&row_as_values).unwrap();
    println!("row_as_values: {:?}", v);

    Ok(row_as_values)
}
