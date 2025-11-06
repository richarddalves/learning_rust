# Compressão de String

Algoritmo de compressão de strings baseado em contagem de caracteres repetidos consecutivamente.

A compressão consiste em contar a frequência de caracteres consecutivos e substituir essas sequências pelo caractere seguido do número de repetições.

## Como executar

```
cargo run
```


## Exemplo

```
$ cargo run

Insira o texto a ser comprimido: aabcccccaaa
Texto comprimido: a2bc5a3
```

```
$ cargo run

Insira o texto a ser comprimido: abcdefgh
Texto comprimido: abcdefgh
```

## Anotações e Aprendizados

Veja [NOTES.md](./NOTES.md)