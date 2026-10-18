# Tradução mecânica do C da decompilação para Rust

Gera `game` (crate `bt3-original`): o jogo inteiro em Rust, traduzido das mesmas fontes
pré-processadas que o port C compila, sobre o modelo de memória do PS2 (endereços originais) e a
aritmética float do PS2 bit a bit (`src/rt/float.rs`, de `port/src/softfloat_ps2.c`).

## Passos

1. `build_ast.sh` (variáveis `DECOMP` e `WORK`): unidades de tradução (`prepare_tus.py`: `#if 1` do
   portsrc, clang `-m32`, constantes float do ee-gcc via `eeconst`), AST JSON e layouts de structs.
   Os layouts vêm de `make_layouts.py` (pasta `layout2/`): o dump `-fdump-record-layouts-complete` do
   clang calcula cedo demais e ignora o `aligned(16)` posto em typedefs (os vetores), então cada struct
   é pedido com `sizeof` e o dump normal é usado; o dump antigo (`layout/`) só cobre os poucos tipos
   locais de função que o novo não alcança. Tudo com `-malign-double` (alinhamento natural do PS2).
2. `translate.sh`: `c2rs/driver.py` indexa e traduz as 7.462 funções (`src/tu/*.rs`); `c2rs/link.py`
   dá os endereços (originais quando conhecidos), os IDs de funções, o despacho de ponteiros de função e
   a imagem de dados (`src/gen`).
3. `cargo build --release` em `game` (15 a 20 min do zero, poucos minutos incremental; o crate
   fica fora do workspace).

`src/rt` (memória, float PS2, operações) e `src/host` (serviços do sistema: memória, arquivos, vblank,
controles, som e GS headless, sondas de teste) são escritos à mão. O resto é gerado.

O que o tradutor reproduz do compilador C, além da semântica da linguagem:
- contas float só com constantes (literais e variáveis `const` com inicializador) são dobradas em IEEE
  com arredondamento ao mais próximo, como o gcc faz em tempo de compilação; o resto usa o float do PS2;
- rótulos asm em funções e variáveis (`extern u32 *gOtCurRead __asm__("gOtCur")`);
- inicializadores com "array filler" do AST JSON do clang (o primeiro item é o preenchimento, os
  seguintes são os elementos).

## Batalha de demonstração (sem imagem)

    BT3_DEMO=1 BT3_TRACE=1 BT3_SEED=0x12345678 BT3_DATA=assets/_legacy/raw \
    BT3_TABLES=assets/data/runtime/source-tables.bt3 target/release/bt3-original > out.log
    python3 tools/translate/compare_traces.py referencia.log out.log   # referencia.log: a mesma demo de antes da mudança

## Estado

- 295 arquivos traduzidos, nenhuma função sem tradução; compila sem erros.
- A demo de batalha é **idêntica à referência nas 4242 linhas do trace** (vblank 187 ao fim: sequência,
  relógio, HP e posições dos dois lutadores).
- Para checar outras situações, use o teste diferencial (`tools/difftest`).
