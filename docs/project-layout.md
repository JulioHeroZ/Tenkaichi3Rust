# Organização do projeto

Um workspace Cargo com duas crates (`game`, `tools/install`), mais ferramentas em Python.

## Raiz

| Arquivo | Para quê |
|---|---|
| `Cargo.toml` | workspace e perfis de compilação |

## `game/`: o jogo (crate `bt3-original`)

| Pasta | Conteúdo |
|---|---|
| `src/tu/` | as unidades de tradução do C, geradas por `tools/translate` (não editar à mão) |
| `src/gen/` | funções, dados e ligações geradas pelo tradutor. Nenhum dado do jogo: as tabelas ficam em `assets/data/tables.txt`, lidas ao abrir (`host/tables.rs`). |
| `src/rt/` | runtime da tradução: memória com os endereços do PS2, float do PS2 bit a bit |
| `src/host/` | os serviços de PC: arquivos e formatos abertos (`assets.rs`, `files.rs`), imagem (`gs/`), som (`snd.rs`), vídeos (`movie.rs`), saves e telas próprias (`saves.rs`, `ui.rs`, `title.rs`), configurações (`settings.rs`) |
| `src/bin/` | ferramentas: `bt3-assetc` (compilador de recursos), `difftest`, `tenkaichi3` (o jogo instalado: janela sem console, acha `assets/` sozinho) |
| `tests/` | testes de integração |
| `meta/` | dados do tradutor; só `_host.json` e `_index.json` entram no Git |

## `tools/`

| Pasta | Conteúdo |
|---|---|
| `install/` | `src/bin/instalador.rs` (a janela do instalador), `src/bin/desinstalar.rs` (Desinstalar.exe), `src/lib.rs` (o instalador de um arquivo só), `instalar.py` (modos jogador e desenvolvedor, os passos), `extract_iso.py` (ISO sem 7-Zip), crate `bt3-install` (cópia dos arquivos e ADX → WAV), `pack.py` (pacotes `.bt3pak` do jogador), `empacotar.py` (gera `dist/Instalar Tenkaichi 3.exe`) |
| `assets/` | `files_extract.py`, `organize.py` (pastas por tipo e nome), `asset_dirs.py` e os conversores de cada formato (texturas, modelos, cenários, animações, efeitos, sons, textos, tabelas tipadas em `layout.py`), recursos de PC (`convert_*.py`) e vídeos (`prepare_pc_media.py`) |
| `translate/` | o tradutor C → Rust (`c2rs/`) e seus scripts |
| `difftest/` | o mesmo C compilado para i386 com o float do PS2, para comparar função por função |

## `assets/` (fora do Git, criada pelo instalador)

| Pasta | Conteúdo |
|---|---|
| `files/` | cada arquivo do jogo em formatos abertos, agrupado por tipo e com nome (`lutadores/Goku (Early)/01423 roupa 1/`, `cenários/12 Kame House/00380 cenário/`...; `tools/assets/organize.py`). O número no começo do nome é como o jogo acha o arquivo. |
| `audio/streams/` | músicas e vozes em WAV |
| `video/` | vídeos de abertura e encerramento: o MPEG-2 do disco, sem conversão (`zs3usop.m2v`, `zs3used.m2v`), decodificado pelo jogo (`host/mpeg2.rs`) |
| `_legacy/raw/` | cópia dos arquivos do disco: entrada da extração e da verificação; o jogo não a lê |

## Outros

* `vendor/windows/`: SDL3 e a dependência dele, copiadas ao lado do executável.
* `docs/`: formatos abertos, saves, controles, renderizador, tabelas, histórico.
