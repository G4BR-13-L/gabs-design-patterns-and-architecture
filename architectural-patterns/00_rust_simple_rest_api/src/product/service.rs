use crate::product::{
    persistence::{find_all, find_one_by_uuid, save},
    product::{Product, ProductInput},
};
use uuid::Uuid;

pub async fn list_products() -> Result<Vec<Product>, tokio_postgres::Error> {
    find_all().await
}

pub async fn find_by_uuid(input_uuid: &str) -> Result<Product, tokio_postgres::Error> {
    println!("Passou 1");
    let uuid: Uuid = Uuid::parse_str(input_uuid).unwrap();
    find_one_by_uuid(uuid).await
}

pub async fn create(input: ProductInput) -> Result<Product, tokio_postgres::Error> {
    let dt = chrono::Utc::now();
    let product = Product {
        uuid: Uuid::new_v4(),
        created_at: dt.naive_utc(),
        name: input.name,
        description: input.description,
        price: input.price,
        available: input.available,
        active: input.active,
    };
    save(product).await
}
