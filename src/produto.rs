#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
}

impl Produto {
    pub fn new(id: u32, nome: &str, categoria: &str) -> Self {
        Produto {
            id,
            nome: nome.to_string(),
            categoria: categoria.to_string(),
        }
    }
}
