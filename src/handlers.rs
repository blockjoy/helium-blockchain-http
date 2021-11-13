use axum::extract::{Extension, Query};
use axum::http::StatusCode;
use serde_json::{json, Value};
use sqlx::postgres::PgPool;

pub struct ParamsTimeRange {
    pub max_time: Option<String>,
    pub min_time: Option<String>,
}

/// Gets the current height of the blockchainn.
///
/// If `max_time` is speified this returns highest block height that was
/// valid (but not equal to) the given time.max_time can be given as an ISO8601
/// timestamp or relative time can be used.
pub async fn get_block_height(
    Extension(pool): Extension<PgPool>,
    Query(time_range): Query<ParamsTimeRange>,
) -> Result<Value, (StatusCode, String)> {
    //TODO: Add support for `max_time` query
    let row: (i64,) = sqlx::query_as("SELECT height FROM blocks ORDER BY hight DESC LIMIT 1")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)?;
    Ok(json!({
        "data": {
            "height": row.0
        }
    }))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
