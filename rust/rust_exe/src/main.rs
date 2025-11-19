use sqlx_lib::error::Error;
use sqlx_lib::queries;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Clearing the database table and adding one row...");

    let id = queries::clear_table_and_add_single_row_async().await?;

    println!("Added one row with id {id}");

    println!("Reading rows from database...");

    let rows = queries::read_all_rows_async().await?;

    println!("{} rows found", rows.len());

    for r in rows {
        println!(
            "Id: {}, Info: {}, Time: {}, Data: {}\n",
            r.id,
            r.info,
            r.time.map_or(String::new(), |t| t.to_string()),
            *r.data
        );
    }

    Ok(())
}
