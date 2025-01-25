use generic_tools::tool_postgres::postgres::{
    tokio_postgres, PostgreSqlConfig, PostgresUtilRequest,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = PostgreSqlConfig {
        username: "ollamachat".to_string(),
        password: "ollamachat".to_string(),
        db_name: "ollamachat".to_string(),
        host: "localhost".to_string(),
    };

    let query = "SELECT 1+1 AS asum".to_string();
    let req = PostgresUtilRequest { query };

    let values = tokio_postgres(&req, config).await;

    let value = match values {
        Ok(val) => format!("{:?}", val),
        Err(e) => {
            println!("error {:?}", e);
            return Err(Box::from(format!(
                "error getting data from database with error: {:?}",
                e
            )));
        }
    };
    let json = serde_json::to_string_pretty(&value).unwrap();
    println!("dbdata \n{}\n", json);

    Ok(())
}
