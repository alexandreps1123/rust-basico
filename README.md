instalar rust linux:
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh

verificar instalação e versão:
$ rustc --version

atualizar rust quando necessário:
$ rustup update

desinstalar rust:
$ rustup self uninstall

acesso a documentação:
$ rustup doc

Compilar:
$ rustc <nome_arquivo>.rs

executar:
$ ./<nome_arquivo>

verificar versão do gerenciador de pacotes do rust:
```
$ cargo --version
```

criar novo diretório e projeto com cargo:
```
$ cargo new <nome_projeto>
```

compila e gera um executável:
```
$ cargo build
```

executar:
```
$ ./target/debug/<nome_projeto>
```

executar de forma direta:
```
$ cargo run
```

verifica se o programa compila mas não gera um executável:
```
$ cargo check
```

gerar uma versão release:
```
$ cargo build --release
```

build documentação de todas as dependências locais e abrir no navegador:
```
$ cargo doc --open
```
