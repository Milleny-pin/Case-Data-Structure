use megastore_search::produto::Produto;
use megastore_search::grafo::GrafoProdutos;
use megastore_search::busca::SistemaBusca;

#[test]
fn test_busca_produto_existente() {
    let mut grafo = GrafoProdutos::new();
    grafo.adicionar_produto(Produto::new(1, "Notebook Gamer", "Eletrônicos"));

    let mut busca = SistemaBusca::new(grafo);
    let resultado = busca.buscar("Notebook");

    assert_eq!(resultado.len(), 1);
    assert_eq!(resultado[0].nome, "Notebook Gamer");
}

#[test]
fn test_busca_produto_inexistente() {
    let mut grafo = GrafoProdutos::new();
    grafo.adicionar_produto(Produto::new(1, "Notebook Gamer", "Eletrônicos"));

    let mut busca = SistemaBusca::new(grafo);
    let resultado = busca.buscar("Celular");

    assert_eq!(resultado.len(), 0);
}
