# Consulta de CEP com API-AWESOMEAPI

Este projeto é uma aplicação web simples construída com o framework Axum em Rust. Ele permite que os usuários consultem informações de endereço 
com base no CEP (Código de Endereçamento Postal) brasileiro, utilizando uma API externa para obter os dados AWESOMEAPI.

## Funcionalidades

- **Consulta de CEP**: Permite que os usuários insiram um CEP e selecionem uma API para buscar informações de endereço.
- **Resultados Dinâmicos**: Os resultados da consulta são exibidos dinamicamente na página, sem a necessidade de recarregar a página.
- **Validação de Entrada**: O sistema valida a entrada do CEP para garantir que ele tenha 8 dígitos.

## Tecnologias Utilizadas

- **Rust**: Linguagem de programação utilizada para desenvolver a aplicação.
- **Axum**: Framework web assíncrono para construção de servidores HTTP.
- **Tokio**: Runtime assíncrono que permite a execução de operações simultâneas.
- **Reqwest**: Cliente HTTP utilizado para fazer requisições a APIs externas.
- **Serde**: Biblioteca para serialização e deserialização de dados em Rust.

## Estrutura do Código

### Dependências

Certifique-se de adicionar as seguintes dependências ao seu `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
axum = "0.6"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
```

## Como Executar

1. **Compile e execute a aplicação**:
   ```bash
   cargo run
   ```

2. **Acesse o aplicativo**:
   Abra seu navegador e vá para `http://127.0.0.1:3000`.
