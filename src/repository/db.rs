use sqlx::{Pool, Postgres, PgPool, Error};
use std::env;
use uuid::Uuid;
use dotenvy::dotenv;

pub async fn get_db_pool() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    // let database_url = "".to_string();  // explicitly set my database url for the sake of demonstrating th etask, normally it would be in the .env
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres")
}