use crate::handlers;
use axum::{routing::get, AddExtensionLayer, Router};
use sqlx::PgPool;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/v1/blocks/height", get(handlers::blocks::get_block_height))
        .layer(AddExtensionLayer::new(pool))
}
