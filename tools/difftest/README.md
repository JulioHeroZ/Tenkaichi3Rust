# Teste diferencial: Rust traduzido x C original

Compara o jogo traduzido (`game`) com o C da decompilação compilado para i386, função por
função e a partir de estados reais do jogo.

## A referência (`build.sh`)

O mesmo C pré-processado que o tradutor lê (`tools/translate/build_ast.sh` gera `tu_out/`), compilado
com clang para i386:
- `-mno-x87 -mno-sse`: toda conta float vira chamada às rotinas do PS2 (`port/src/softfloat_ps2.c`);
- `-ftrivial-auto-var-init=zero`: variáveis locais sem inicialização valem 0, como no Rust (sem isso o
  lixo da pilha aparece como diferença);
- `fidify.py`: no LLVM IR, todo uso de endereço de função vira o ID da função (o endereço do PS2 que o
  Rust usa), então ponteiros de função guardados na memória valem o mesmo nos dois programas; as
  chamadas continuam diretas e as indiretas chegam ao código por trampolins postos nos IDs;
- as tabelas de dados do jogo ficam nos endereços do Rust (`--defsym`, `gen_stubs.py`) e as demais
  globais do C são sincronizadas pelo mapa do link (`mapping.py`);
- `harness.c`: programa sem libc que recebe comandos pela entrada padrão (mapear, escrever, ler memória,
  chamar função). O `difftest` do crate (`src/bin/difftest.rs`) aceita o mesmo protocolo.

## Ferramentas

- `dt.py <regex>`: chama as funções cujo nome casa com a expressão, com entradas aleatórias, nos dois
  programas e compara retorno e memória.
- `BT3_PROBE=<função>:<n>[,...]` no jogo traduzido grava a memória inteira antes e depois da n-ésima
  chamada, os argumentos, as contagens de chamadas e as chamadas filhas (`probe_*`).
- `replay.py` / `twin.py`: executam uma chamada sondada na referência (ou nos dois) a partir desse
  estado e comparam a memória.
- `lockstep.py [--seed S] 1,100,300`: grava várias chamadas de `Battle_Update` numa execução e compara
  cada uma com a referência (memória inteira); `--descend` desce na primeira que divergir.
- `descend.py <função> <n>`: começa numa chamada que diverge e desce pelas chamadas filhas até a função
  cujo próprio corpo diverge.

Caminhos: um `config_local.py` (fora do git) ou as variáveis `BT3_GEN` (crate), `BT3_REF` (saída do build), `BT3_TABLES`, `BT3_DATA`, `BT3_DIFF_WORK`.

## Bugs que isso achou

Layouts de structs com vetores `aligned(16)`, inicializadores com "array filler", variável com rótulo
asm (`gOtCurRead`) e dobra de constantes float: depois das correções a demo de batalha ficou idêntica à
referência.
