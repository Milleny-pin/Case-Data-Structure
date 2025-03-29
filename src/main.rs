mod produto;
mod grafo;

use produto::Produto;
use grafo::GrafoDeProdutos;

fn main() {
    let mut grafo = GrafoDeProdutos::new();

    let p1 = Produto::new(1, "Notebook", "Eletrônicos");
    let p2 = Produto::new(2, "Smartphone", "Eletrônicos");
    let p3 = Produto::new(3, "Mouse", "Periféricos");

    grafo.adicionar_produto(p1);
    grafo.adicionar_produto(p2);
    grafo.adicionar_produto(p3);

    grafo.adicionar_relacao(1, 2);
    grafo.adicionar_relacao(1, 3);

    if let Some(produto) = grafo.buscar_produto(1) {
        println!("Produto encontrado: {:?}", produto);
    }

    let relacionados = grafo.produtos_relacionados(1);
    println!("Produtos relacionados ao ID 1:");
    for produto in relacionados {
        println!("{:?}", produto);
    }
}
