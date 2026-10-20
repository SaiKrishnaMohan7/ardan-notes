use axum::{Router, extract::State, routing::get};
use sqlx::{Pool, Row, Sqlite};
use tracing::{info, subscriber::set_global_default};

async fn setup_logging() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_target(true)
        .finish();
    set_global_default(subscriber)?;
    Ok(())
}

async fn setup_db_pool() -> anyhow::Result<Pool<Sqlite>> {
    info!("Setting up db pool");
    let url = std::env::var("DATABASE_URL")?;
    Ok(sqlx::SqlitePool::connect(&url).await?)
}

async fn test(State(pool): State<sqlx::SqlitePool>) -> String {
    let result = sqlx::query("SELECT COUNT(id) FROM images")
        .fetch_one(&pool)
        .await
        .unwrap();
    let count = result.get::<i64, _>(0);
    format!("{count} images in the database")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging().await?;
    info!("Logging setup done...");
    dotenv::dotenv()?;
    info!("Env setup done...");
    let pool = setup_db_pool().await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations run!");

    let app = Router::new().route("/", get(test)).with_state(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
