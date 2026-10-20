use sqlx::{FromRow, Pool, Row, Sqlite};
use tracing::{info, subscriber::set_global_default};

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

async fn setup_logging() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_target(true)
        .finish();
    set_global_default(subscriber)?;
    Ok(())
}

async fn setup_db_pool(url: &str) -> anyhow::Result<Pool<Sqlite>> {
    info!("Setting up db pool");
    Ok(sqlx::SqlitePool::connect(url).await?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging().await?;
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;
    let pool = setup_db_pool(&db_url).await?;

    // let messages = sqlx::query("SELECT id, message FROM messages")
    //     .map(|row: sqlx::sqlite::SqliteRow| {
    //         let id: i64 = row.get(0);
    //         let message: String = row.get(1);
    //         (id, message)
    //     })
    //     .fetch_all(&pool)
    //     .await?;

    let messages = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
        .fetch_all(&pool)
        .await?;

    info!("DB chat supz/");

    println!("{messages:?}");

    Ok(())
}
