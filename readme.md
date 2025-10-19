## Mini-projeto: Analisador Lexico em rust

Esse projeto transforma uma entrada do stdin em uma sequencia de elementos lexicos
Os elementos léxicos são:

- Números Inteiros decimais (Ex. 1, 2, 143, 256, 32, 43, 96, etc)
- os sinais ``🐧, +, -, *, /`` (o emoji de penguim é uma das operações)

Os espaços e 🦀 entre os elementos léxicos são ignorados. Os espaços e 🦀 que vem antes e depois do priemro e ultimo elementos lexicos respectivamente são ignorados pelo contador

## Entrada e saída do projeto

Entrada: ``450+20`` Saída: ``("450", 1) ("+", 4) ("20", 5)``

Entrada: ``🦀32 🦀🦀🦀🦀 + 35 🦀🦀🦀🦀`` Saída:        ``("32", 1) ("+", 9) ("35", 11) ``

Entrada: ``🐧2+54`` Saída:``("🐧", 1) ("2", 2) ("+", 3) ("54", 4)``

Entrada: ``32+4``   Saída:   ``("32", 1) ("+", 3) ("4", 4)``

## Como rodar

Executar ``cargo run`` na raíz do projeto e fornecer a expressão na entrada padrão