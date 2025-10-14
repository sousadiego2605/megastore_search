# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

##  Descrição do Projeto

Este é um projeto da faculdade para a matéria de Estrutura de Dados. O objetivo era criar um sistema de busca rápido para a loja "MegaStore", usando a linguagem Rust.

O sistema de busca antigo da loja era lento, então a gente usou a estrutura de **Índice Invertido** pra conseguir encontrar produtos no catálogo de forma quase instantânea.

##  Tecnologias Utilizadas

* **Linguagem:** Rust
* **Gerenciador de Pacotes:** Cargo
* **Bibliotecas:** `serde` (para lidar com os dados do produto)

##  Como Executar o Projeto

Pra rodar o projeto, siga os passos no seu terminal:

1.  **Clone o repositório:**
    ```bash
    git clone [https://github.com/SEU_USUARIO/megastore_search.git](https://github.com/SEU_USUARIO/megastore_search.git)
    cd megastore_search
    ```

2.  **Execute o programa de exemplo:**
    ```bash
    cargo run
    ```
    Isso vai compilar e rodar o código principal, que já tem alguns produtos e buscas de exemplo.

 # Como Executar os Testes

Para rodar os testes e verificar se a lógica de busca está funcionando certo, use o comando:
```bash
cargo test
```

# Exemplos de Uso

Ao rodar `cargo run`, o programa vai mostrar os seguintes exemplos no terminal:

**Busca por "notebook gamer":**
```
Resultados encontrados: [
    Product {
        id: 1,
        name: "Notebook Gamer Pro",
        ...
    },
]
```

**Busca por "jogos sem fio":**
```
Resultados encontrados: [
    Product {
        id: 3,
        name: "Headset Gamer Sem Fio",
        ...
    },
]
```

Buscando por: 'notebook para jogos' (a palavra 'para' é ignorada)
Resultados encontrados: [
    Product {
        id: 1,
        name: "Notebook Gamer Pro",
        ...
    },
]
```

# Arquitetura do Sistema

O projeto foi dividido em alguns arquivos pra ficar mais organizado:
* `src/main.rs`: Programa principal que roda a demonstração.
* `src/lib.rs`: Define a parte "biblioteca" do projeto, pra que os testes e o `main` possam usar os outros módulos.
* `src/engine.rs`: É o coração do projeto. Tem a lógica do motor de busca.
* `src/models.rs`: Define como é a `struct` de um Produto.
* `src/tokenizer.rs`: src/tokenizer.rs: Tem a função de tokenização, que processa os textos: converte para minúsculas, quebra em palavras e remove 'stop words' (palavras comuns como 'e', 'de', 'para') para tornar a busca mais precisa.

# Algoritmos e Estruturas de Dados

A parte principal do projeto são as estruturas de dados que a gente escolheu pra garantir a velocidade:

1.  **Índice Invertido:**
    * **Como funciona:** Usamos um `HashMap` onde cada chave é uma palavra (ex: "notebook") e o valor é um conjunto com os IDs de todos os produtos que têm essa palavra.
    * **Por que é rápido:** Buscar uma chave em um `HashMap` é uma operação muito rápida, com complexidade **O(1)** em média.

2.  **Repositório de Produtos:**
    * **Como funciona:** Outro `HashMap` que simplesmente guarda todos os produtos, usando o ID como chave. Isso serve para pegar os dados completos do produto no final da busca.

O **algoritmo de busca** segue os seguintes passos:
1.  A consulta do usuário é processada pelo `tokenizer`. Este processo **converte o texto para minúsculas, quebra a frase em palavras-chave (tokens) e remove *stop words*** (palavras comuns como 'e', 'para', 'um', etc.).
2.  Para cada token relevante, o conjunto de IDs de produtos correspondente é recuperado do Índice Invertido.
3.  É calculada a **interseção** entre todos os conjuntos de IDs para encontrar os produtos que contêm *todos* os termos importantes da busca.
4.  No final, os dados completos dos produtos são buscados no repositório principal usando os IDs resultantes.

# Desempenho e Escalabilidade

* **Velocidade:** O sistema é rápido porque não precisa olhar produto por produto a cada busca. Ele vai direto nas palavras-chave do índice.
* **Memória:** O ponto negativo é que tudo fica guardado na memória RAM. Isso deixa o sistema rápido, mas consome mais memória.
* **Escalabilidade:** Para um site com milhões de produtos, o próximo passo seria salvar o índice em disco em vez de na memória, ou dividir o índice em vários servidores.

---
Projeto acadêmico feito para a matéria de Estrutura de Dados.
