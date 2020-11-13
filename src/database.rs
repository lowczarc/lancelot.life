use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use sqlx::{Pool, Postgres};

pub async fn mysql_connection() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env!("DATABASE_URL"))
        .await
        .unwrap();

    println!("Connected to PostgreSQL");


    pool.execute("CREATE TABLE IF NOT EXISTS goals (
        id int not null,
        content text not null
    )").await.unwrap();

    pool.execute("CREATE TABLE IF NOT EXISTS influences (
        id int not null,
        name varchar(255) not null,
        link text
    )").await.unwrap();

    pool
}
