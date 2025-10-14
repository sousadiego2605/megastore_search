// src/main.rs

// Agora, usamos os componentes da nossa própria biblioteca
use megastore_search::models::Product;
use megastore_search::engine::SearchEngine;

fn main() {
    println!("Inicializando a MegaStore Search Engine...");
    let mut engine = SearchEngine::new();

    let products = vec![
        Product { id: 1, name: "Notebook Gamer Pro".to_string(), category: "Eletrônicos".to_string(), brand: "PowerPC".to_string(), description: "Um notebook poderoso para jogos.".to_string() },
        Product { id: 2, name: "Smartphone Android Top".to_string(), category: "Eletrônicos".to_string(), brand: "MobileX".to_string(), description: "Celular com câmera de alta resolução.".to_string() },
        Product { id: 3, name: "Headset Gamer Sem Fio".to_string(), category: "Acessórios".to_string(), brand: "SoundBlast".to_string(), description: "Fone de ouvido sem fio para jogos.".to_string() },
    ];

    for product in products {
        engine.index_product(product);
    }
    println!("Produtos indexados com sucesso!");

    let query1 = "notebook gamer";
    println!("\nBuscando por: '{}'", query1);
    let results1 = engine.search(query1);
    println!("Resultados encontrados: {:#?}", results1);

    let query2 = "jogos sem fio";
    println!("\nBuscando por: '{}'", query2);
    let results2 = engine.search(query2);
    println!("Resultados encontrados: {:#?}", results2);
}