# Contribuir

## Regras do projeto

* **Nada do jogo no repositório.** Texturas, sons, tabelas e o executável vêm do disco de cada pessoa. Fica fora do
  Git: `assets/` (inclusive `assets/data/tables.txt`, as tabelas do jogo que o executável lê ao abrir).
* **Fidelidade primeiro.** O jogo tem que se comportar como o original. Toda mudança passa por estes testes:
  * a verificação dos arquivos (`BT3_VERIFY_FILES`) dá 0 diferenças;
  * a luta de demonstração dá o mesmo registro quadro a quadro.
* **Formatos abertos.** Dados novos que o jogo lê vão em formatos comuns (PNG, glTF, WAV, JSON, texto), com um
  conversor em `tools/assets` e a remontagem em `game/src/host/assets.rs`. Documente em
  [docs/open-formats.md](docs/open-formats.md).
* **Mensagens de commit e documentação em português.**

## Onde mexer

| Quero... | Lugar |
|---|---|
| mudar um serviço de PC (janela, som, controles, saves, vídeos) | `game/src/host/` |
| corrigir o código traduzido do jogo | `tools/translate/c2rs`. Gere de novo: `game/src/tu` e `game/src/gen` são saída do tradutor e não se editam à mão. |
| um formato aberto novo ou melhor | `tools/assets/` (conversor) + `game/src/host/assets.rs` (remontagem) |
| o instalador | `tools/install/` (janela: `src/bin/instalador.rs`; passos: `instalar.py`) |

## Compilar e rodar a partir do código

* O instalador: `cargo run --release -p bt3-install --bin instalador` (rodado do projeto, usa o Python do sistema e
  compila o jogo com cargo). O instalador para distribuir: `python tools/install/empacotar.py`.
* O jogo, com os dados de uma instalação de desenvolvedor:
  `set BT3_ASSETS=C:\Program Files (x86)\Tenkaichi 3\assets` e `cargo run --release -p bt3-original --bin tenkaichi3`
  (copie `vendor\windows\*.dll` para `target\release`).

## Testes

Rode a partir da raiz, com `assets` apontando para os dados instalados (ou a pasta `assets` do projeto).

**Arquivos refeitos dos formatos abertos.** Tem que dar "0 differ":

```
set BT3_VERIFY_FILES=1
set BT3_DATA=assets\_legacy\raw
set BT3_ASSETS=assets
target\release\bt3-original
```

**Luta de demonstração.** O registro tem que ser igual ao de antes da mudança:

```
set BT3_DEMO=1
set BT3_TRACE=1
set BT3_SEED=0x12345678
set BT3_PC_ONLY=1
set BT3_ASSETS=assets
target\release\bt3-original > luta.txt
```

**Testes diferenciais contra o C** da decompilação: veja [tools/difftest/README.md](tools/difftest/README.md).
