use sqlx::{Pool, Postgres};
use std::env;

pub async fn create_db_pool() -> Pool<Postgres> {
  let url = env::var("DATABASE_URI").expect("DATABASE_URL not set");
  let pool = sqlx::postgres::PgPoolOptions::new()
    .connect(&url)
    .await
    .expect("Cannot connect to PostgreSQL");

  println!("PostgreSQL success!");
  pool
}