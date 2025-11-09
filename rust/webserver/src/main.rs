use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
};
use dotenvy::dotenv;
use sqlx::{PgPool, Pool, Postgres};
use sqlx_lib::{RowData, read_all_rows};
use tokio::net::TcpListener;

use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable};

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::fmt::Display,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[utoipa::path(
        get,
        path = "/",
        responses(
            (status = 200, description = "index", body = String),
        ),
    )]
async fn get_default() -> String {
    "🦀🐱🤡".to_string()
}

#[utoipa::path(
        get,
        path = "/health",
        responses(
            (status = 200, description = "Returns a string from the database to confirm availability", body = String),
            (status = 500, description = "Internal server error", body = String),
        ),
    )]
async fn get_health_check(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'db says: i am health ⚕️'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

#[utoipa::path(
        get,
        path = "/all",
        responses(
            (status = 200, description = "Returns all rows from the database", body = Vec<RowData>),
            (status = 500, description = "Internal server error", body = String),
        ),
    )]
async fn get_all_data(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<RowData>>, (StatusCode, String)> {
    let rows = read_all_rows(&pool).await.map_err(internal_error)?;

    Ok(Json(rows))
}

const TAG: &str = "rust_webserver";

fn build_router() -> Router<Pool<Postgres>> {
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = TAG, description = "Some API")
        )
    )]
    struct ApiDoc;

    let app_router = OpenApiRouter::new()
        .routes(routes!(get_default))
        .routes(routes!(get_health_check))
        .routes(routes!(get_all_data));

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(app_router)
        .split_for_parts();

    let router = router.merge(Scalar::with_url("/scalar", api));

    router
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file if we have it
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let router = build_router().with_state(pool);

    // todo, remove unwraps and handle errors properly
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
