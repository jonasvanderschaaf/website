#[macro_use]
extern crate rocket;

use rocket::State;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[get("/")]
fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/database")]
async fn test_database(pool: &State<Pool<Postgres>>) -> &'static str {
    pool.acquire().await.unwrap();

    "Success"
}

#[launch]
async fn rocket() -> _ {
    let database_url = "postgresql://postgres@postgres:5432/database";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .unwrap();

    rocket::build()
        .manage(pool)
        .mount("/", routes![hello_world, test_database])
}
