use std::collections::HashMap;
use crate::produto::Produto;
use crate::grafo::GrafoProdutos;

pub struct SistemaBusca {
    pub grafo: GrafoProdutos,
    pub cache: HashMap<String, Vec<Produto>>,
}

impl SistemaBusca {
    pub fn new(grafo: GrafoProdutos) -> Self {
        SistemaBusca {
            grafo,
            cache: HashMap::new(),
        }
    }

    pub fn buscar(&mut self, termo: &str) -> Vec<Produto> {
        if let Some(resultado) = self.cache.get(termo) {
            return resultado.clone();
        }

        let mut encontrados = Vec::new();
        for produto in self.grafo.produtos.values() {
            if produto.nome.contains(termo) {
                encontrados.push(produto.clone());
            }
        }

        self.cache.insert(termo.to_string(), encontrados.clone());
        encontrados
    }
}
