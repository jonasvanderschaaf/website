#[macro_use]
extern crate rocket;

use rocket::State;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use rocket_dyn_templates::Template;

use std::collections::HashMap;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/database")]
async fn test_database(pool: &State<Pool<Postgres>>) -> &'static str {
    pool.acquire().await.unwrap();

    "Success"
}

#[get("/template")]
async fn test_template() -> Template {
    let mut context = HashMap::new();

    context.insert("abc", "abcd");

    Template::render("template", context)
}

#[launch]
async fn rocket() -> _ {
    let database_url = "postgresql://postgres@postgres:5432/database";

    // Create a pool of connections to the postgres database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .unwrap();

    rocket::build()
        // Add the database pool to the state of Rocket
        .manage(pool)
        // Attach the Tera templating fairing to Rocket
        .attach(Template::fairing())
        .mount("/", routes![hello_world, test_database, test_template])
}
