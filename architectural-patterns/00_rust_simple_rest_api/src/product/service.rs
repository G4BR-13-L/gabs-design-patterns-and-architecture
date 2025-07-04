use crate::product::persistence::find_all;

pub fn list_products() -> String {
    let products = find_all();
    products.join(", ")
}
