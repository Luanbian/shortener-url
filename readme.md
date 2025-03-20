# Encurtador de URL

Um encurtador de urls feito na linguagem Rust

## 🚀 Instalando

Para instalar o encurtador de url, siga estas etapas:
> clone o repositório
> 
> abra o repositório em sua máquina
```
  cargo build
```
> Agora precisamos rodar o nosso banco de dados postgres
>
> ainda no terminal, execute:
```
  docker compose up -d
```

---

## ☕ Usando o encurtador de url

Para usar o encurtador de url, siga estas etapas:

> vá para a pasta do projeto
```
  cargo run
```

---

## 🧰 Testando manualmente

Para testar manualmente o encurtador de url, siga estas etapas:
> após rodar cargo run no seu computador
>
> veja a mensagem de listening que aparece, ela indicará a porta em que está rodando a aplicação
>
> Exemplo:
```
  listening on port 127.0.0.1:8080
```
> 127.0.0.1 indica o endereço IP que é localhost
> 
> 8080 indica a porta que a nossa aplicação está rodando

### Agora no seu terminal (Ou qualquer outra ferramenta para fazer requisições)

> Você fará uma requisição HTTP do tipo POST
>
> O tipo enviado deve ser json
>
> a chave deve ser original_url e o valor deve ser a url longa completa
```
  curl -X POST http://127.0.0.1:8080 -H "Content-Type: application/json" -d '{"original_url": "https://play.rust-lang.org/?version=stable&mode=debug&edition=2024"}'
```
> Você terá um link como resposta
>
> Exemplo
```
http://localhost:8080/WCW7Yz
```
> Ao clicar, você será redirecionado para o mesmo local da url original!

---

## 🤖 testando via código

Para testar via código o encurtador de url, siga estar etapas:
> após rodar cargo run no seu computador
>
> em outro terminal rode
```
  cargo test
```
> No terminal que está rodando a aplicação você verá um "short_url" com a url encurtada
>
> Exemplo
```
  short_url: http://localhost:8080/WCW7Yz
```
> Ao copiar e colar o link no seu navegador você será redirecionado para o mesmo local da url original!
