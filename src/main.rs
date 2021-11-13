use axum;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "helium-blockchain-http=debug")
    }
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test")
        .await
        .expect("Should be able to connect to database.");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(routes::app(pool).into_make_service())
        .await
        .unwrap();
}
