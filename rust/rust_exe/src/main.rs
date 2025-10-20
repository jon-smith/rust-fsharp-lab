#[tokio::main]
async fn main() -> Result<(), sqlx_lib::Error> {
    println!("Clearing the database table and adding one row...");

    let id = sqlx_lib::clear_table_and_add_single_row_async().await?;

    println!("Added one row with id {}", id);

    println!("Reading rows from database...");

    let rows = sqlx_lib::read_all_rows_async().await?;

    println!("{} rows found", rows.len());

    for r in rows {
        println!(
            "Id: {}, Info: {}, Time: {}, Data: {}\n",
            r.id,
            r.info,
            r.time.map(|t| t.to_string()).unwrap_or(String::from("")),
            *r.data
        );
    }

    Ok(())
}
