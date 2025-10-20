use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Json;
use sqlx::types::chrono;
use std::env;
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Debug)]
pub struct RowData {
    pub id: i32,
    pub info: String,
    pub data: Json<serde_json::Value>,
    pub time: Option<chrono::NaiveDateTime>,
}

pub use sqlx::Error;

pub async fn read_all_rows(pool: &PgPool) -> Result<Vec<RowData>, sqlx::Error> {
    let rows = sqlx::query_as!(RowData, "SELECT * FROM datatable")
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

pub async fn clear_table_and_add_single_row_async() -> Result<i32, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    sqlx::query!("TRUNCATE datatable").execute(&pool).await?;

    let info = String::from("info");
    let date_time = chrono::Utc::now().naive_utc();
    let data = json!({
        "name": "wiseguy",
        "number": 43,
        "serial-numbers": [
            "x30495mmn",
            "pp-jj6654"
        ]
    });

    let rec = sqlx::query!(
        r#"
    INSERT INTO datatable ( info, data, time )
    VALUES ( $1, $2, $3 )
    RETURNING id
            "#,
        info,
        Json(data) as _,
        date_time,
    )
    .fetch_one(&pool)
    .await?;

    Ok(rec.id)
}

pub async fn read_all_rows_async() -> Result<Vec<RowData>, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    read_all_rows(&pool).await
}

pub fn read_all_rows_sync() -> Result<Vec<RowData>, sqlx::Error> {
    Runtime::new().unwrap().block_on(read_all_rows_async())
}
