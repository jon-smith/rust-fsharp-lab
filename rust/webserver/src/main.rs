use axum::{Router, extract::State, http::StatusCode, routing::get};
use dotenvy::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::fmt::Display,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

async fn health_check(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'db says: i am health ⚕️'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file if we have it
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .route("/", get(|| async { "🦀🐱🤡" }))
        .route("/health", get(health_check))
        .with_state(pool);

    // todo, remove unwraps and handle errors properly
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
