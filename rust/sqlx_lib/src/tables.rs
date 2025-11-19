use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::types::chrono;
use utoipa::{ToSchema, schema};

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub struct RowData {
    pub id: i32,
    pub info: String,
    #[schema(value_type = serde_json::Value)]
    pub data: Json<serde_json::Value>,
    pub time: Option<chrono::NaiveDateTime>,
}
