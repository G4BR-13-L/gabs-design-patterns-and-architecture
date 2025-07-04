#[macro_use]
extern crate rocket;

use rocket::{get, routes, Rocket};
use rust_simple_rest_api::configuration::migrations::{
    check_table_exists, connect_to_db, create_migration_table, run_migrations,
};
use rust_simple_rest_api::product::controller::product_routes;
use std::fs;
use std::path::Path;

#[launch]
async fn rocket() -> _ {
    let client = connect_to_db()
        .await
        .expect("Error connecting to the database");

    if !check_table_exists(&client)
        .await
        .expect("Error checking for migration table")
    {
        create_migration_table(&client)
            .await
            .expect("Error creating migration table");
    }

    run_migrations(&client)
        .await
        .expect("Error running migrations");

    rocket::build().mount("/", product_routes())
}
