use anyhow::Ok;
use futures::TryStreamExt;
use sqlx::{FromRow, Pool, Sqlite, migrate};
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

async fn update_message(id: i64, message: String, pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    sqlx::query("UPDATE messages SET message = ? WHERE id = ?")
        .bind(message)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging().await?;
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;
    let pool = setup_db_pool(&db_url).await?;

    // Auto migrate
    migrate!("./migrations").run(&pool).await?;

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

    println!("{:?} {:?}", messages[0].id, messages[0].message);

    update_message(2, "Updated!".to_string(), &pool).await?;

    let mut message_stream =
        sqlx::query_as::<_, Message>("SELECT id, message FROM messages").fetch(&pool);

    while let Some(message) = message_stream.try_next().await? {
        println!(
            "Look ma, I am Streaming: {:?} {:?}",
            message.id, message.message
        )
    }

    Ok(())
}
