use crate::product::service::list_products;
use rocket::{get, post, routes, Route};

#[get("/products")]
fn list() -> String {
    list_products()
}

#[post("/products")]
fn create() -> &'static str {
    "Product created"
}

pub fn product_routes() -> Vec<Route> {
    routes![list, create]
}
