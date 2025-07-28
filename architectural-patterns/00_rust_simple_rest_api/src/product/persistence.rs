use crate::configuration::db::connect_to_db;
use crate::product::product::Product;
use tokio_postgres::{types::ToSql, Error};

pub fn find_all() -> Vec<String> {
    vec!["Product A".to_string(), "Product B".to_string()]
}

pub async fn save(product: Product) -> Result<Product, Error> {
    let client = connect_to_db().await?;

    client
        .execute(
            "INSERT INTO t_product (uuid, name, description, price, created_at, active, available)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &product.uuid,        // Uuid
                &product.name,        // String
                &product.description, // String
                &product.price,       // i32
                &product.created_at,  // chrono::NaiveDateTime
                &product.active,      // bool
                &product.available,   // bool
            ],
        )
        .await?;

    Ok(product)
}
