# Máximo divisor comum

Um programa para descobrir o MDC entre dois números.

## Como executar

```shell
cargo run
```

O programa pede dois números `a` e `b` e calcula o MDC de duas formas: por decomposição em fatores primos (não eficiente e pode travar para números muito grandes) e pelo algoritmo euclidiano.

## Exemplo:

```bash
$ cargo run

Insira o valor a: 12
Insira o valor b: 20

O máximo divisor comum entre 12 e 20 é 4.
Segundo o algoritmo euclidiano é: 4.
```

```bash
$ cargo run

Insira o valor a: 432
Insira o valor b: 6789

O máximo divisor comum entre 432 e 6789 é 3.
Segundo o algoritmo euclidiano é: 3.
```

```bash
$ cargo run

Insira o valor a: 100
Insira o valor b: 1000

O máximo divisor comum entre 100 e 1000 é 100.
Segundo o algoritmo euclidiano é: 100.
```

## Aprendizado e explicações

Veja [NOTES.md](./NOTES.md)