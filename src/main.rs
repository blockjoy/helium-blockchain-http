use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "helium-blockchain-http=debug")
    }
    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL should be defined.");

    let db_max_connections: u32 = std::env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .expect("Could not parse DATABASE_MAX_CONNECTIONS");

    let db_min_connections: u32 = std::env::var("DATABASE_MIN_CONNECTIONS")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .expect("Could not parse DATABASE_MIN_CONNECTIONS");

    let db_conn_timeout: u64 = std::env::var("DATABASE_CONN_TIMEOUT")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .expect("Could not parse DATABASE_CONN_TIMEOUT");
    let db_conn_timeout = Duration::from_secs(db_conn_timeout);

    let db_conn_idle_timeout: u64 = std::env::var("DATABASE_CONN_IDLE_TIMEOUT")
        .unwrap_or_else(|_| "600".to_string())
        .parse()
        .expect("Could not parse DATABASE_CONN_IDLE_TIMEOUT");
    let db_conn_idle_timeout = Duration::from_secs(db_conn_idle_timeout);

    let db_conn_max_lifetime: u64 = std::env::var("DATABASE_CONN_MAX_LIFETIME")
        .unwrap_or_else(|_| "1800".to_string())
        .parse()
        .expect("Could not parse DATABASE_CONN_MAX_LIFETIME");
    let db_conn_max_lifetime = Duration::from_secs(db_conn_max_lifetime);

    let pool = PgPoolOptions::new()
        .min_connections(db_min_connections)
        .max_connections(db_max_connections)
        .connect_timeout(db_conn_timeout)
        .idle_timeout(db_conn_idle_timeout)
        .max_lifetime(db_conn_max_lifetime)
        .connect(&db_url)
        .await
        .expect("Should be able to connect to database.");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(routes::app(pool).into_make_service())
        .await
        .unwrap();
}
