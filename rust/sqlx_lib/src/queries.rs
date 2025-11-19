use dotenvy::dotenv;
use serde_json::json;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Json;
use sqlx::types::chrono;
use tokio::runtime::Runtime;

use crate::env_var_helper::get_env_var;
use crate::error::Error;
use crate::tables::RowData;

pub async fn read_all_rows(pool: &PgPool) -> Result<Vec<RowData>, Error> {
    #[allow(
        clippy::panic,
        clippy::missing_panics_doc,
        reason = "query_as! false panic"
    )]
    let rows = sqlx::query_as!(RowData, "SELECT * FROM datatable")
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

pub async fn clear_table_and_add_single_row_async() -> Result<i32, Error> {
    dotenv().ok();

    let database_url = get_env_var("DATABASE_URL")?;

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

    #[allow(
        clippy::panic,
        clippy::missing_panics_doc,
        reason = "query_as! false panic"
    )]
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

pub async fn read_all_rows_async() -> Result<Vec<RowData>, Error> {
    dotenv().ok();

    let database_url = get_env_var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    read_all_rows(&pool).await
}

pub fn read_all_rows_sync() -> Result<Vec<RowData>, Error> {
    Runtime::new()?.block_on(read_all_rows_async())
}
