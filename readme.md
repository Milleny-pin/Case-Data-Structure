# 📚 **README - Sistema de Recomendação de Produtos - MegaStore**  

---

## 🚀 **Descrição do Projeto**  
Este projeto é um **Sistema de Recomendação de Produtos** desenvolvido em **Rust** para otimizar a busca e recomendação de itens no catálogo da **MegaStore**. Utilizando a estrutura de **Grafos**, o sistema permite relacionar produtos e fornecer recomendações rápidas e precisas.  

---

## 🛠 **Tecnologias Utilizadas**  
- **Rust** - Linguagem de Programação de Alto Desempenho  
- **Cargo** - Gerenciador de Pacotes e Build  
- **Crate std::collections** - Uso de `HashMap` e `HashSet` para estrutura de dados  

---

## 🟢 **Funcionalidades**  
- Cadastro de Produtos com ID, Nome e Categoria  
- Criação de Relações entre Produtos  
- Busca por Produto Específico  
- Recomendação de Produtos Relacionados  

---

## 📂 **Estrutura de Pastas**  
├── src │ ├── main.rs // Arquivo principal de execução │ ├── produto.rs // Módulo de definição de Produto │ ├── grafo.rs // Módulo de estrutura de Grafos └── Cargo.toml // Configuração do projeto Rust
---

## ⚙️ **Como Executar o Projeto**  

### ✔️ Pré-Requisitos  
Certifique-se de ter o **Rust** e o **Cargo** instalados.  
Você pode instalar através do [Rustup](https://www.rust-lang.org/tools/install) com o comando:  
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
