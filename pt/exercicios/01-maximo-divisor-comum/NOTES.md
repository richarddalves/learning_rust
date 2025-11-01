# Anotações e aprendizados

## Enunciado

> ​Dado duas variáveis a = 15 e b = 40, faça um programa para determinar o máximo divisor comum entre eles

## Processo de resolução

Comecei tentando resolver o problema matematicamente / logicamente.

Para encontrar o MDC entre dois ou mais números, precisamos encontrar os divisores de todos, e o MDC será o maior divisor que é comum a todos. Até aí é o que é conhecimento comum.

A forma mais conhecida de fazer isso é, ao invés de dividir cada número por divisores comuns e depois encontrar o maior divisor que apareça na lista de divisores de todos os números, dividir apenas por números primos. Isso é a fatoração por números primos.
A partir disso, o MDC será igual a múltiplicação de todos os fatores que aparecem em comum.

Isso é mais prático e rápido do que encontrar cada divisor real de cada número e ter o enome trabalho de verificar se o maior divisor desse número é também divisor de todos os outros números ao qual estamos comparando, e caso seja, se também é o maior deles.

Por exemplo, para fatorar 12 e 20:

```
12 | 2
6  | 2
3  | 3
1

20 | 2
10 | 2
5  | 5
1
```

Fatores de 12: [2, 2, 3]. <br>
Fatores de 20: [2, 2, 5].

Para determinar o MDC basta múltiplicar todos os fatores que aparecem em comum a todos. Nesse caso, temos apenas dois `2`. Logo, $mdc(12, 20) = 2 * 2 = 4$.

Conclusão: Como o MDC é o maior divisor comum, pensei em decompor ambos os números `a` e `b` em fatores primos, depois criar uma lista com os fatores que aparecem em ambos e múltiplicar todos os elementos dessa lista.

---

### Implementei várias funções auxiliares:

#### 1) Para verificar se um número é primo:

```rust
fn is_primo(num: u32) -> bool
```

> Obs.: Não foi totalmente escrita por mim. Cheguei a ela através de pesquisas por como descobrir se um número é primo ou não. Todas as outras foram escritas por mim.

#### 2) Para gerar uma lista de números primos até n:

```rust
fn primos_ate(n: u32) -> Vec<u32>
```

Para decompor um número em fatores primos, utiliza-se apenas números primos para a fatoração.

Por exemplo, no exemplo acima para fatorar 12:

```
12 | 2
6  | 3
3  | 3
1
```

Note que para fatorar no papel, sabemos praticamente naturalmente qual o próximo fator usar. Primeiro o 2 até não ter mais como divir por 2, depois o 3 até não ter mais como dividir por 3, etc... Mas e no código? Eu deveria manualmente dividir por 2, por 3, por 5...? Mas e se o número a ser fatorado for muito grande, até qual divisor eu deveria programar?

Não me pareceu a melhor forma de fazer isso, porque além de não respeitar o princípio DRY, não seria possível cobrir todos os casos possíveis.

Então, pensei que fosse ser importante ter uma lista de todos os números primos até o número `n`, para a hora que eu estivese fatorando esse `n` em fatores primos, não precisar escrever manualmente cada divisor e sim ir percorrendo a lista e dividindo automáticamente. Me parecia a forma mais idiomática e inteligente de resolver.

> Obs.: Logo descobri que não era, e que na verdade, é uma solução bem ruim porque a função tem complexidade $O(n * \sqrt{n})$.

#### 3) Para gerar a lista de números primos até n:

```rust
fn decompor_em_fatores_primos(mut num: u32) -> Vec<u32>
```

#### 4) Para pedir um número ao usuário:

```rust
fn pedir_u32(msg: &str) -> u32
```

Criei essa função para auxiliar a abstrair o código para ficar mais limpo. Não fiz quaisquer verificações de segurança e deixei a conversão da entrada para inteiro para o `.parse()`.

---

### Descobrindo o Algoritmo Euclidiano

Enquanto pesquisava, descobri o algoritmo euclidiano. É baseado na ideia que MDC(a, b) = MDC(b, a mod b).

É muito mais simples e eficiente do que a minha solução original. A complexidade é $O(log(n))$.

A partir disso, criei a versão `mdc_euclides()`.

#### Diferença prática:

| Valor de `a` ou `b` | Minha solução original | Algoritmo de Euclides |
| ------------------- | ---------------------- | --------------------- |
| 10                  | ~ 31 operações         | ~ 4 operações         |
| 100                 | ~ 1000 operações       | ~ 7 operações         |
| 1.000               | ~ 31.000 operações     | ~ 10 operações        |
| 10.000              | ~ 1.000.000 operações  | ~ 14 operações        |

### Bugs encontrados

Enquanto escrevia o código tive pequenos bugs, mas todos dentro do normal que fui resolvendo enquanto ia debugando.

O mais marcante foi usar `2..n` ao invés de `2..=n` em `primos_ate()`, que fazia o próprio `n` não ser incluído na lista, e, enquanto tentava encontrar a origem do problema, acabei estourando a memória fazendo o loop entrar em um loop infinito.


### Conclusão

Por fim, acabei aprendendo algumas coisas valiosas. Tentei ir documentando o código com `///` enquanto escrevia, abstrair a lógica para funções por funcionalidades diferentes para deixar o código mais limpo, "auto documentado", e menos repetitivo, e também a deixar o meu código mais eficiente, quando parei para pensar sobre a complexidade dele em notação *Big O*.

As próprias experiências de estar escrevendo essa documentação e ir documentando o código enquanto escrevia também foram muito valiosas e sinto que estou evoluindo.