# Anotações e Aprendizados

## Processo de Aprendizado

Minha primeira abordagem foi fazer um loop externo percorrendo todos os caracteres da string e então, verificar se o caracter seguido é repetido. Caso seja, iniciar uma contagem para saber quantos caracteres tem repetidos em sequência e depois pular o índice do loop para depois do ultimo caractere que foi contado como repetido.

Essa foi apenas a minha primeira intuição, mas não é a melhor possível. Tem complexidade n² ou n³, pois o uso de `chars().nth()` por si só já percorre toda a string, não sendo então a melhor escolha nesse cenário.

Uma solução melhor e com complexidade O(n) seria converter a string para um array de caracteres diretamente, dessa forma a busca pelo elemento no array seria melhor do que pelo caractere na string. Optei por não implementar e sim apenas demonstrar minha primeira abordagem e aprendizado.

## Observações

Utilizei a crate [`stdin-helper`](https://github.com/richarddalves/stdin-helper) que foi desenvolvida por mim mesmo para abstrair a lógica de entrada de texto pelo terminal.