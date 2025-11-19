use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
};
use dotenvy::dotenv;
use sqlx::{PgPool, Pool, Postgres};
use sqlx_lib::queries;
use sqlx_lib::tables::RowData;
use tokio::net::TcpListener;

use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable};

use sqlx_lib::env_var_helper::get_env_var;
use sqlx_lib::error::Error;

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
    let rows = queries::read_all_rows(&pool)
        .await
        .map_err(internal_error)?;

    Ok(Json(rows))
}

const TAG: &str = "rust_webserver";

#[allow(clippy::needless_for_each, reason = "OpenApi macro uses it")]
fn build_router() -> (Router<Pool<Postgres>>, utoipa::openapi::OpenApi) {
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

    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(app_router)
        .split_for_parts()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env file if we have it
    dotenv().ok();

    let database_url = get_env_var("DATABASE_URL")?;

    let pool = sqlx::PgPool::connect(&database_url).await?;

    let (router, api) = build_router();

    let router = router
        .merge(Scalar::with_url("/scalar", api))
        .with_state(pool);

    // todo, remove unwraps and handle errors properly
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    // This test will fail if the generated OpenAPI spec does not match the checked-in version
    // If you intentionally changed the API, replace the openapi.json file with the openapi-tmp.json output
    fn test_build_router_and_export_openapi() {
        let (_, api) = build_router();

        // Create output directory for OpenAPI docs
        let output_dir = "../openapi";
        if let Err(e) = fs::create_dir_all(output_dir) {
            assert!(false, "Failed to create output directory: {e}");
        }

        // Export OpenAPI spec as JSON
        let json_spec = match api.to_pretty_json() {
            Ok(spec) => spec,
            Err(e) => {
                assert!(false, "Failed to serialize OpenAPI spec to JSON: {e}");
                return;
            }
        };

        let output_json_path = Path::new(output_dir).join("openapi-tmp.json");
        if let Err(e) = fs::write(&output_json_path, json_spec.clone()) {
            assert!(false, "Failed to write OpenAPI JSON file: {e}");
        }

        // Verify file was created and is not empty
        assert!(
            output_json_path.exists(),
            "OpenAPI JSON file was not created"
        );

        // Read in the current source file for comparison
        let input_json_path = Path::new(output_dir).join("openapi.json");
        let file_content = match fs::read_to_string(&input_json_path) {
            Ok(content) => content,
            Err(e) => {
                assert!(false, "Failed to read OpenAPI JSON file: {e}");
                return;
            }
        };

        assert_eq!(
            file_content
                .parse::<serde_json::Value>()
                .expect("Failed to parse file_content as JSON"),
            json_spec
                .parse::<serde_json::Value>()
                .expect("Failed to parse json_spec as JSON"),
            "OpenAPI JSON does not match expected content. 
            The OpenAPI spec may have changed.
            If this is expected, the openapi.json should be updated."
        );
    }
}
