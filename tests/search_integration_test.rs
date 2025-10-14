use megastore_search::models::Product;
use megastore_search::engine::SearchEngine;

#[test]
fn test_index_and_search() {
    let mut engine = SearchEngine::new();
    let product1 = Product { id: 101, name: "Teclado Mecânico RGB".to_string(), category: "Periféricos".to_string(), brand: "GamerGear".to_string(), description: "Teclado para gamer.".to_string() };
    engine.index_product(product1.clone());

    let results = engine.search("teclado gamer");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, 101);

    let results_empty = engine.search("mouse");
    assert!(results_empty.is_empty());
}