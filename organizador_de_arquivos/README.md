# Organizador de Arquivos (CLI em Rust)

Uma ferramenta de linha de comando (CLI) simples e eficiente, escrita em Rust, para organizar arquivos em um diretório com base em suas extensões.

## Funcionalidades

-   **Organização Automática:** Varre um diretório e move arquivos para subpastas categorizadas (ex: `Imagens`, `Documentos`, `Videos`).
-   **Performance:** Construída em Rust para ser "blazing fast" e segura.
-   **Fácil de Usar:** Interface de linha de comando simples, utilizando a biblioteca `clap`.

## Como Usar

1.  Clone o repositório:
    ```bash
    git clone [https://github.com/SEU_USUARIO/organizador_de_arquivos.git](https://github.com/SEU_USUARIO/organizador_de_arquivos.git)
    ```

2.  Navegue até o diretório:
    ```bash
    cd organizador_de_arquivos
    ```

3.  Compile o projeto:
    ```bash
    cargo build --release
    ```

4.  Execute a ferramenta, apontando para o diretório que deseja organizar:
    ```bash
    ./target/release/organizador_de_arquivos --diretorio /caminho/para/sua/pasta_de_downloads
    ```

## Motivação

Este projeto foi desenvolvido como um exercício prático para aprofundar os conhecimentos em Rust, focando em:

-   Manipulação do sistema de arquivos (`std::fs`).
-   Criação de CLIs robustas com `clap`.
-   Uso de estruturas de dados como `HashMap`.