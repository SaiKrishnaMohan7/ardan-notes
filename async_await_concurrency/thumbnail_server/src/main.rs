use std::path::Path;

use axum::{
    Form, Json, Router,
    body::Body,
    extract::{self, Multipart, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Row, Sqlite};
use tokio::task::spawn_blocking;
use tokio_util::io::ReaderStream;
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

async fn home() -> Html<String> {
    let content = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(content)
}

async fn uploader(State(pool): State<sqlx::SqlitePool>, mut multipart: Multipart) -> Html<String> {
    let mut tags = None;
    let mut image = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "tags" => tags = Some(String::from_utf8(data.to_vec()).unwrap()),
            "image" => image = Some(data.to_vec()),
            _ => panic!("Unknown field name: {name}"),
        }
    }

    if let (Some(tags), Some(image)) = (tags, image) {
        // Destructuring both Options at once
        let image_id = insert_image_into_db(&pool, &tags).await.unwrap();
        save_image(image_id, &image).await.unwrap();
        // CPU-heavy resize off the async workers; the response doesn't wait for it
        spawn_blocking(move || {
            make_thumbnail(image_id).unwrap();
        });
    } else {
        panic!("Missing field");
    }

    let content = tokio::fs::read_to_string("src/redirect.html").await.unwrap();
    Html(content)
}

async fn insert_image_into_db(pool: &Pool<Sqlite>, tags: &str) -> anyhow::Result<i64> {
    let row = sqlx::query("INSERT INTO images (tags) VALUES (?) RETURNING id")
        .bind(tags)
        .fetch_one(pool)
        .await?;

    Ok(row.get(0))
}

async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
    let base_path = Path::new("images");
    if !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }

    let image_path = base_path.join(format!("{id}.jpg"));
    if image_path.exists() {
        anyhow::bail!("File already exists");
    }

    tokio::fs::write(image_path, bytes).await?;

    Ok(())
}

/// Streams a file from `images/` as a JPEG; 404 if it isn't there (yet: thumbnails are made in the background).
async fn serve_jpeg(filename: String) -> Response {
    let Ok(file) = tokio::fs::File::open(&filename).await else {
        return StatusCode::NOT_FOUND.into_response();
    };
    (
        [
            (header::CONTENT_TYPE, "image/jpeg".to_string()),
            (header::CONTENT_DISPOSITION, format!("filename={filename}")),
        ],
        Body::from_stream(ReaderStream::new(file)),
    )
        .into_response()
}

// `extract::Path` (URL segment), not `std::path::Path` imported above
async fn get_image(extract::Path(id): extract::Path<i64>) -> Response {
    serve_jpeg(format!("images/{id}.jpg")).await
}

async fn get_thumbnail(extract::Path(id): extract::Path<i64>) -> Response {
    serve_jpeg(format!("images/{id}_thumb.jpg")).await
}

fn make_thumbnail(id: i64) -> anyhow::Result<()> {
    let image_bytes = std::fs::read(format!("images/{id}.jpg"))?;
    let image = image::load_from_memory(&image_bytes)?; // guesses the format from the bytes

    image.thumbnail(100, 100).save(format!("images/{id}_thumb.jpg"))?;

    Ok(())
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
struct ImageRecord {
    id: i64,
    tags: String,
}

async fn list_images(State(pool): State<sqlx::SqlitePool>) -> Json<Vec<ImageRecord>> {
    sqlx::query_as::<_, ImageRecord>("SELECT id, tags FROM images ORDER BY id")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into()
}

#[derive(Deserialize)]
struct Search {
    tags: String,
}

// `Form` reads the body, so it goes last, like `Multipart`
async fn search_images(State(pool): State<sqlx::SqlitePool>, Form(form): Form<Search>) -> Html<String> {
    let tag = format!("%{}%", form.tags);

    let rows = sqlx::query_as::<_, ImageRecord>(
        "SELECT id, tags FROM images WHERE tags LIKE ? ORDER BY id",
    )
    .bind(tag)
    .fetch_all(&pool)
    .await
    .unwrap();

    let results: String = rows
        .iter()
        .map(|row| format!("<a href=\"/image/{0}\"><img src='/thumb/{0}' /></a><br />", row.id))
        .collect();

    let content = tokio::fs::read_to_string("src/search.html").await.unwrap();
    Html(content.replace("{results}", &results))
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

    let app = Router::new()
        .route("/test", get(test))
        .route("/", get(home))
        .route("/image/{id}", get(get_image))
        .route("/thumb/{id}", get(get_thumbnail))
        .route("/images", get(list_images))
        .route("/upload", post(uploader))
        .route("/search", post(search_images))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    info!("Listening on port 3000");
    axum::serve(listener, app).await?;

    Ok(())
}
