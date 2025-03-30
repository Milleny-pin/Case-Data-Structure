use std::collections::{HashMap, HashSet};
use crate::produto::Produto;

pub struct GrafoDeProdutos {
    pub produtos: HashMap<u32, Produto>,
    pub adjacencias: HashMap<u32, HashSet<u32>>,
}

impl GrafoDeProdutos {
    pub fn new() -> Self {
        GrafoDeProdutos {
            produtos: HashMap::new(),
            adjacencias: HashMap::new(),
        }
    }

    
    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.produtos.insert(produto.id, produto.clone()); 
        self.adjacencias.entry(produto.id).or_insert(HashSet::new());
    }

    pub fn adicionar_relacao(&mut self, id_origem: u32, id_destino: u32) {
        if let Some(vizinhos) = self.adjacencias.get_mut(&id_origem) {
            vizinhos.insert(id_destino);
        }
    }

    pub fn buscar_produto(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    pub fn produtos_relacionados(&self, id: u32) -> Vec<&Produto> {
        match self.adjacencias.get(&id) {
            Some(vizinhos) => vizinhos
                .iter()
                .filter_map(|&id_vizinho| self.produtos.get(&id_vizinho))
                .collect(),
            None => vec![],
        }
    }
}
