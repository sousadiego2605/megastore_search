use std::collections::{HashMap, HashSet};
use crate::models::Product;
use crate::tokenizer::tokenize;

pub struct SearchEngine {
    store: HashMap<u64, Product>,
    inverted_index: HashMap<String, HashSet<u64>>,
}

impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            store: HashMap::new(),
            inverted_index: HashMap::new(),
        }
    }

    pub fn index_product(&mut self, product: Product) {
        let product_id = product.id;
        let text_to_index = format!(
            "{} {} {} {}",
            product.name, product.category, product.brand, product.description
        );

        let tokens = tokenize(&text_to_index);

        for token in tokens {
            self.inverted_index
                .entry(token)
                .or_default()
                .insert(product_id);
        }

        self.store.insert(product_id, product);
    }

    pub fn search(&self, query: &str) -> Vec<&Product> {
        let tokens = tokenize(query);
        if tokens.is_empty() { return vec![]; }

        let mut result_ids: HashSet<u64> = match self.inverted_index.get(&tokens[0]) {
            Some(ids) => ids.clone(),
            None => return vec![],
        };

        for token in tokens.iter().skip(1) {
            if let Some(ids) = self.inverted_index.get(token) {
                result_ids = result_ids.intersection(ids).cloned().collect();
            } else {
                return vec![];
            }
        }

        result_ids
            .iter()
            .filter_map(|id| self.store.get(id))
            .collect()
    }
}