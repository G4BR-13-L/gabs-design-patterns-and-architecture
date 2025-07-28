use crate::product;
use crate::product::product::{Product, ProductInput};
use crate::product::service;
use chrono::NaiveDateTime;
use rocket::{get, post, routes, serde::json::Json, Route};

#[get("/products")]
fn list() -> String {
    service::list_products()
}

#[post("/products", format = "json", data = "<input>")]
async fn create(input: Json<ProductInput>) -> String {
    let input = input.into_inner();

    let product: Product = service::create(input).await;

    format!("Product created {}", product.uuid)
}

pub fn product_routes() -> Vec<Route> {
    routes![list, create]
}
