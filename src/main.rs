use dotenvy::var;
use pgvector::Vector;
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url =
        var("DATABASE_URL").map_err(|e| format!("Failed to get DATABASE_URL: {}", e))?;
    let database_url = database_url.as_str();

    let gemini_api_token =
        var("GEMINI_API_TOKEN").map_err(|e| format!("Failed to get GEMINI_API_TOKEN: {}", e))?;
    let gemini_api_token = gemini_api_token.as_str();

    let content = var("MESSAGE").unwrap_or("Hello world".to_string());
    let content = content.as_str();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| format!("DB connection failed: {}", e))?;

    let reqw_client = reqwest::Client::new();
    let reqw_response = reqw_client
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/embedding-001:embedContent?key={gemini_api_token}"
        ))
        .json(&json!({
        "model": "models/embedding-001",
        "content": {
            "parts":[{
                 "text": content
            }]
        }
        }))
        .send()
        .await?;
    let reqw_response = reqw_response
        .text()
        .await
        .map_err(|e| format!("Failed to read reqwest response body: {e}"))?;
    let reqw_response: Value = serde_json::from_str(&reqw_response)
        .map_err(|e| format!("Could not parse response body: {e}"))?;
    // println!("reqw_response: {reqw_response}");
    let reqw_response: Value = reqw_response
        .get("embedding")
        .ok_or("No embedding")?
        .to_owned();
    let embedding: Vec<f32> = reqw_response
        .get("values")
        .ok_or("No values")?
        .as_array()
        .ok_or("values are not an array")?
        .iter()
        .map(|v| v.as_f64().ok_or("Value is not a float").map(|v| v as f32))
        .collect::<Result<Vec<f32>, &str>>()?;

    let embedding = Vector::from(embedding);
    println!("message \"{content}\" encoded successfully");

    if let Ok((id, res)) = sqlx::query_as::<_, (i64, String)>(
        r#"SELECT id, content FROM items WHERE content != $1 ORDER BY embedding <-> $2 LIMIT 1"#,
    )
    .bind(content)
    .bind(&embedding)
    .fetch_one(&db_pool)
    .await
    {
        println!("search result: \"{res}\" with id {id}");
    } else {
        println!("no search result");
    }

    sqlx::query(
        "INSERT INTO items
            (content, embedding)
            VALUES ($1, $2);
            ;",
    )
    .bind(content)
    .bind(embedding)
    .execute(&db_pool)
    .await?;
    println!("added to the db");

    Ok(())
}
