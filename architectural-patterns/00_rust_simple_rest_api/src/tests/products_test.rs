#[cfg(test)]
mod products_test {
    use crate::product::persistence; // Adjust the path as needed.

    #[test]
    fn test_find_all_returns_products() {
        let products = persistence::find_all();
        assert_eq!(products.len(), 2);
        assert_eq!(products[0], "Product A");
        assert_eq!(products[1], "Product B");
    }

    #[test]
    fn test_find_all_does_not_return_empty_list() {
        let products = persistence::find_all();
        assert!(!products.is_empty());
    }
}
