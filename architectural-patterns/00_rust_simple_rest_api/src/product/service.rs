use crate::product::{
    persistence::{find_all, save},
    product::{Product, ProductInput},
};
use uuid::Uuid;

pub fn list_products() -> String {
    let products = find_all();
    products.join(", ")
}

pub async fn create(input: ProductInput) -> Product {
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
    save(product).await.unwrap()
}
