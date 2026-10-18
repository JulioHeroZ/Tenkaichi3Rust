# Histórico do projeto

O registro do trabalho, do mais recente ao mais antigo. A organização atual está em
[project-layout.md](project-layout.md).




## Pacote de texturas do PCSX2, New Game depois de Options, menus esticados (Claude, 18/10/2026)

- Texturas HD: o pacote do Wii (PLAYANDO, Dolphin) deu lugar a pacotes do PCSX2, achados como o port em C acha
  (`port/src/gs/gs_texpack.c` do Tenkaichi3Decomp): XXH3 dos blocos crus da memória do GS e da paleta
  (`host/xxh3.rs`, conferido contra o xxhash.h em 3002 entradas). Sem conversão de paletas IA8 nem escolha por
  semelhança: some o Piccolo esbranquiçado da tela inicial, a caixa de texto branca e o retrato branco da seleção.
  Na nuvem, 1639 arquivos do pacote: tela inicial, menu principal e Dragon History em HD. O instalador baixa o
  pacote do MediaFire (`texturas.py`) e apaga o pacote antigo.
- New Game não saía da tela inicial depois de abrir Options por ela: a tela de opções cria um fluxo de save próprio
  (`McFlow_Init`) e o apaga ao fechar (`gMcFlow = NULL`), levando junto o da tela inicial; o save era gravado, mas o
  aviso de "pronto" nunca chegava. `run_options` (`host/title.rs`) devolve o fluxo da tela inicial. Reproduzido e
  conferido na nuvem.
- Menus: esticados para a proporção escolhida, no lugar da imagem 4:3 sobre a cópia desfocada (pedido do usuário).

## Conversão mais rápida (Claude, 18/10/2026)

- Medido onde o tempo da instalação vai: a maior parte é o `pzs3us2` (65.201 vozes e músicas em ADX, 1,1 GB) e os
  sons dos bancos (ADPCM do PS2), decodificados e conferidos amostra por amostra em Python.
- `adxconv.py` e `vagconv.py`: a decodificação num laço só, e a reconstrução (a conferência de que o arquivo volta
  igual) com numpy sobre o som inteiro, com a volta ao laço antigo quando um WAV foi editado. `texconv.py`: busca
  de endereços por tabela no lugar de ordenar. `bpe.py`: expansão por tabela.
- Saída idêntica byte a byte: 361 arquivos com bancos de som, 699 arquivos BPE, um décimo do `pzs3us1` e 600
  vozes reais do `pzs3us2` (no computador do usuário), com WAVs editados também iguais. Tempo: ADX 6,7x, sons dos
  bancos ~5x, amostra do `pzs3us1` 2,8x.
- A barra de progresso conta todos os arquivos vistos (antes, só os convertidos, contra o total de todos: andava
  devagar demais). `DATA_VERSION` passou a ser fixa (em `instalar.py`), para uma mudança só de velocidade não
  forçar a reinstalação.

## Atualizar sem refazer, e teste da build (Claude, 18/10/2026)

- A 0.2.0 saía com a imagem da janela quebrada (o centro de uma cor só): o `uvScale` do shader de apresentação era
  posto com o shader do fundo em uso. Os testes não pegaram porque, sem janela, `present()` não desenhava nada;
  agora `BT3_SHOT_WINDOW=LxA` salva a imagem como a janela a mostra, pelo mesmo código (`draw_window`).
- `tools/install/testar_build.py` (`testar_build.cmd`): compila o jogo como o instalador leva, joga a tela inicial e
  uma luta numa janela escondida (`BT3_HIDDEN`, `BT3_EXIT_AT`) e falha se o jogo der erro ou se o centro da imagem
  for de uma cor só. `gerar_instalador.cmd` testa antes de empacotar, em segundo plano.
- Atualização: `assets/instalacao.txt` guarda a versão dos dados (`instalar.data_version`: hash das ferramentas de
  conversão, de `pack.py`, do export e do layout das tabelas) e o tipo. Com a mesma versão e o mesmo tipo, o
  instalador só troca os programas e mantém os dados (sem ISO). A janela acha a instalação anterior pelo registro,
  mostra "Atualizar" e a caixa "Refazer tudo do zero". O ícone, numa atualização do jogador, vem do `.ico` já feito.
- Na nuvem: o teste da build passou (tela inicial e luta); a decisão atualizar/instalar conferida com pastas falsas.
  A janela foi conferida só pelo compilador.

## Desinstalador (Claude, 18/10/2026)

- `Desinstalar.exe` na pasta do jogo (`tools/install/src/bin/desinstalar.rs`): confirma, pergunta se apaga os saves,
  se copia para %TEMP% e apaga a pasta do jogo (só se tiver `Tenkaichi3.exe`), os atalhos e a entrada da lista de
  programas. `instalar.py` registra o jogo em Configurações > Aplicativos (HKLM, ou HKCU sem administrador).

## Instalador de um arquivo só, com janela (Claude, 18/10/2026)

- `Instalar Tenkaichi 3.exe` (`tools/install/src/bin/instalador.rs`, Win32 sem bibliotecas): escolha da ISO, do tipo
  (Jogar / Desenvolvedor) e da pasta (padrão `Program Files (x86)\Tenkaichi 3`, pede administrador), barra de
  progresso, registro, cancelar, "Jogar agora". Os dois tipos instalam `Tenkaichi3.exe` e atalhos; Jogar empacota
  os dados, Desenvolvedor deixa os arquivos editáveis (pasta liberada para o grupo Usuários).
- O instalador carrega dentro de si o Python embutido, as ferramentas e o jogo compilado (`bt3_install::payload`,
  zlib), desempacotados em %TEMP%; `empacotar.py` o gera. `instalar.cmd` e `jogar.cmd` saíram.
- Nuvem: os dois modos de `instalar.py` testados do começo ao fim; o formato do pacote embutido testado (Python
  escreve, Rust lê, arquivos idênticos). A janela foi conferida só pelo compilador (sem Windows na nuvem).

## Pacotes, vídeos em Rust e instalador com dois modos (Claude, 18/10/2026)

- `host/vfs.rs`: o jogo lê os arquivos abertos soltos ou de pacotes `assets/*.bt3pak` (zlib onde compensa); um
  arquivo solto vale mais que o do pacote, então um mod é um arquivo posto no lugar. `tools/install/pack.py` faz os
  pacotes: na nuvem, 3,7 GB de arquivos abertos viraram ~1 GB; verificação 3333/3333 e luta de demonstração
  idênticas lendo só dos pacotes, pelo executável do jogador.
- Vídeos: `host/mpeg2.rs` decodifica o MPEG-2 do disco em Rust (os filmes só têm quadros I); `pss_video.py` separa o
  vídeo do PSS sem converter (120 + 179 MB, no lugar dos 800 MB em FFV1). Todos os 7010 quadros dentro de 1 do FFmpeg.
  O FFmpeg não é mais necessário nem no jogo nem na instalação.
- Instalador: modo jogador (pasta de jogo com `Tenkaichi3.exe`, DLLs e `assets/*.bt3pak`, atalho na área de
  trabalho, arquivos de trabalho apagados no fim) e modo desenvolvedor. `tools/install/empacotar.py` gera
  `dist/Tenkaichi3-PC.zip` (programas prontos + Python embutido com numpy), sem nenhum dado do jogo.

## Executável sem dados do jogo (Claude, 18/10/2026)

- As tabelas do jogo saíram do programa: `gen/tables.rs` não existe mais; o instalador escreve
  `assets/data/tables.txt` e o jogo o lê ao abrir. O executável compila sem o disco e é o mesmo para todos.
- Novo executável `tenkaichi3` (game/src/bin/tenkaichi3.rs): janela sem console, acha `assets/` ao lado de si e joga
  só com os recursos de PC; o que ele relata vai para `Documentos\My Games\Tenkaichi3Rust\log.txt`.
- Nuvem: luta de demonstração idêntica lendo tables.txt, também pelo executável novo aberto de outra pasta.

## Pastas dos assets com nome (Claude, 17/10/2026)

- `tools/assets/organize.py` agrupa `assets/files` por tipo e dá nome às pastas: `lutadores/<personagem>/` (roupas,
  modelos danificados, animações, efeitos das técnicas, vozes, retrato, dados), `cenários/<nn nome>/` (cenário, tela
  dividida, sons, imagem de seleção), `menus/`, `sons/`, `textos/`, `roteiros/`... O que é cada arquivo vem dos ids
  de arquivo da decompilação; os nomes dos personagens e cenários, dos textos do próprio jogo.
- O jogo acha cada arquivo pelo número no começo do nome (`file_dirs` em `host/assets.rs`); os links glTF entre pastas
  (animações → modelo) são reescritos. O instalador roda isso no passo 5.
- Nuvem: verificação 3333/3333 sem diferenças e luta de demonstração idêntica com as pastas organizadas.

## Reorganização (Claude, 17/10/2026)

- Saiu tudo o que era do projeto anterior: o executável híbrido C + Rust (`engine/`, a crate raiz
  `tenkaichi3-rust`, `tools/migration`), a prévia nativa (`Previa.cmd`, scripts `.ps1`, os exportadores antigos de
  `tools/`), os documentos dele, o vigia de testes (`vigia.cmd`, `teste.cmd`) e as pastas antigas de `assets`. Está
  tudo no histórico do Git (até o commit 5257306).
- `game/original` virou `game/`, e `jogar.cmd` foi para a raiz. O workspace Cargo tem o jogo e o instalador. O
  `SDL3.dll` fica em `vendor/windows`.
- O passo 3 do instalador (cópia dos arquivos e ADX → WAV) virou a crate própria `bt3-install`, sem depender do
  código antigo. O WAV sai idêntico ao de antes.
- Teste de instalação do zero na nuvem: extração completa (3333 arquivos) e recursos de PC com os mesmos números do
  Windows. A luta de demonstração só com arquivos de PC é a mesma, quadro a quadro; o carregamento leva 8 quadros a
  menos.

## Antes: continuidade revisada (17/10/2026)

## Instalação limpa com a extração nova (Claude, 17/10/2026)

- Os conversores do passo 5 do instalador (`convert_native_audio_banks.py`, `convert_native_text.py`,
  `convert_native_lips.py`, `convert_empty_directories.py`) liam `skeleton.bin`, que a extração nova não grava mais.
  Agora usam `tools/assets/native_source.py`, que refaz o que o skeleton guardava a partir de `skeleton.txt` e dos
  registros tipados (byte order marks dos textos, cabeçalhos LIPS).
- Bancos de som: a última amostra de cada banco começa com blocos zerados que `vagconv.py` deixa de fora
  (`sound_<n>_tailN.wav`); o conversor grava `vagi_NNN.wav` com o silêncio na frente, igual amostra a amostra ao WAV
  antigo, sem precisar da cópia legada em `assets/audio/sfx`.
- Diretórios vazios: a pasta só com `file-tables.json` + `skeleton.txt` vira `empty-directory.json`.
- `convert_native_motion_sources.py` grava cada arquivo inteiro ou nada (pode ser interrompido).
- Teste na nuvem: extração nova + passo 5 num subconjunto (texto, lips, banco de som, roteiro, diretórios vazios,
  roster, grupos de pontos, lutador com banco de motions): verificação Rust 0 diferenças. No Windows, o passo 5 sobre
  a extração existente dá os mesmos números de antes (100 tabelas de texto, 96 lips, 361 bancos, 18924 amostras).

## Nenhum skeleton.bin (Claude, 17/10/2026)

- Nenhuma pasta de `assets/files` tem mais `skeleton.bin` nem `entry_NNN.bin` com dados: o tamanho do arquivo fica em
  `skeleton.txt` e cada byte vem de um arquivo aberto. Verificação Rust: 0 diferenças; demo com o trace idêntico.
- Novos formatos (docs/open-formats.md): `stage-chains.json` e `stage-layout.json` (cadeias VIF e tabelas dos
  cenários, com nós animados), `movies.json` (os 433 filmes FOD dos menus), `texture-packets.txt` +
  `texture-entries.json`, `camera-tables.json`, escalas do ADX no `adx.txt`, cabeçalhos LIPS, tabelas de texto dentro
  das entradas, entradas de motion descritas em `motion-sources.json` (cerca de 57 mil pastas a menos), texturas dos
  pacotes de efeito (EPCK) dos cenários.
- O que ainda não tem estrutura conhecida está em `file-tables.json` como registros `Words` (lista de palavras de
  32 bits, no lugar exato). Em pzs3us1: **~6,6 MB** (17 MB antes de superfícies, modelos aninhados, câmeras aninhadas, sons, colisão do corpo e mais tipos de efeito) (eram 83 MB quando os skeletons foram removidos). Depois disso
  ganharam estrutura: tabelas de jogo dos lutadores (`model-tables.json`: golpes, ki blasts, movimento, técnicas,
  habilidades, IA, cortes de câmera, física de cabelo), efeitos (`effect-tables.json`: emissores, partículas,
  animações V000; `effects.gltf`), paletas alternativas, sincronia labial e textos aninhados. Resta: efeitos das
  técnicas ~5 MB (dados dos outros tipos de parte), menus ~3,4 MB, cenários ~3,3 MB, arquivos de modelo ~3 MB
  (entradas 1 e 0x15), bancos 031–033 ~2 MB.
- 17/10 à tarde: jogo aberto no Windows com a extração nova, aparentemente tudo certo (teste do usuário).
- 17/10 à tarde: jogo testado no Windows com a extração nova (menus, luta, cenários): tudo certo.
- Ferramentas no lugar (rodam numa extração existente, retomáveis): `add_tables.py`, `add_adx_scales.py`,
  `add_motion_sources.py`, `add_tex_packets.py`, `add_rest.py`, `refresh.py`.

## Instalador e motions dos lutadores (Claude, 17/10/2026)

- `instalar.cmd <iso>` (tools/install): lê a ISO sem 7-Zip, gera `gen/tables.rs` direto do executável
  (`tables_from_elf.py`, 2388 runs iguais às tabelas tipadas), copia/converte streams, extrai tudo e roda os
  conversores de PC e de vídeo. Cada passo pronto é pulado.
- `fmotionconv.py`: as motions dentro dos arquivos de lutador (entradas 0x1C..0x23, mesmo formato das dos bancos) em
  `model-motions.gltf/.bin/.json`; o Rust monta cada entrada (`model_motions_apply`). Aplicado em 723 arquivos.
- Bytes originais nos skeletons: **35,7 MB** (cenários 16,6; lutadores 8,4; texturas/layouts 10; outros 0,7).

## Configurações do PC com F1 (Claude, 17/10/2026)

- `host/settings.rs`: tela cheia, resolução interna (1x a 4x; vale ao abrir o jogo) e as teclas de cada botão do
  controle e do analógico esquerdo, em `Documentos\My Games\Tenkaichi3Rust\settings.ini` (texto simples, editável;
  `BT3_SETTINGS` usa outro arquivo). F1 abre a tela sobre a imagem pausada; F11 também grava a tela cheia.
  Esc, F1 e F11 ficam reservadas. `BT3_SCALE` continua tendo precedência sobre o arquivo.

## Colisão dos cenários em formato aberto (Claude, 16/10/2026)

Medição dos `skeleton.bin`: cerca de 160 MB de bytes originais ainda estavam neles; 110 MB nos 74 cenários, e
quase tudo isso era a colisão (zonas, malhas, polígonos e árvores de caixas).

- `tools/assets/colconv.py`: a colisão de cada cenário vai para `collision.gltf` + `collision.bin` (abre em
  qualquer programa 3D); o resto que o jogo precisa fica nos extras. Reconstrução em Rust: `collision_apply`.
- `stageconv.py`: as palavras w e q dos vértices dos cenários também saem (`_W_GAME`, `_Q_GAME`).
- Aplicado nos 74 cenários já extraídos (`add_collision.py`, `add_stage_wq.py`): 79,4 + 14,5 MB de bytes
  originais a menos. Cada um conferido byte a byte antes de gravar; verificação Rust (`BT3_VERIFY_FILES`) nos
  cenários disponíveis na nuvem: 0 diferenças. Falta rodar a verificação completa no Windows.
- `modelconv.py`: a palavra q dos vértices dos modelos de lutador também sai (`_Q_GAME`); aplicado nos modelos já
  extraídos com `add_model_q.py` (cerca de 9,6 MB). Verificação Rust na nuvem: 0 diferenças.
- Total medido depois disso: **57,4 MB** de bytes originais nos skeletons (eram ~160 MB): cenários 16,7 MB, lutadores
  29,5 MB, texturas/layouts 10 MB, outros 1,3 MB.
- **Modelos dos lutadores montados do glTF** (`pmdlconv.py`, `model_build` em Rust): `model.json` guarda o resto do
  modelo (registros dos nós, cabeçalhos dos lotes); o Rust escreve o modelo inteiro e as partes. Editar triângulos e
  mudar o número de vértices funciona (re-strip automático; o arquivo cresce até o limite do buffer do jogo,
  0xCE000). Teste: lutador com 97 primitivas reordenadas → luta de demonstração com imagens idênticas pixel a pixel
  e rastro idêntico. Aplicado nos 741 modelos (`add_model_layout.py`).
- Total depois disso: **40,4 MB** de bytes originais nos skeletons (cenários 16,6; lutadores 12,8; texturas 10; outros 1).
- Dois skeletons foram truncados por uma execução interrompida (02264, 02807) e extraídos de novo; as ferramentas agora
  gravam de forma atômica (arquivo temporário + rename).
- O que sobra nos skeletons: ~16 MB nos cenários (cabeçalhos VIF/GIF, tabelas do cenário), ~39 MB nos 698
  arquivos de lutador (entradas 28/29/35 de dados de jogo ainda sem significado conhecido, nós do modelo,
  cabeçalhos VIF), ~10 MB de layouts de textura e menus.

## Esc com confirmação e vibração do controle (Claude, 16/10/2026)

- **Esc** não fecha mais o jogo direto: pausa e pergunta "Sair do jogo?" (Sim/Não) sobre a última imagem
  (`host/gs/gpu.rs: quit_question`, desenho em `host/ui.rs: show_question`). Setas trocam, Enter/K/✕ confirma,
  Esc/L/○ volta ao jogo. Fechar a janela continua saindo na hora. F11 continua alternando tela cheia.
- **Vibração**: `sceVibSetActParam` (chamado pelo `Pad_SendActuator` do jogo a cada quadro) passa o byte do
  DualShock (motor grande 0..127 << 1 | motor pequeno) para `SDL_RumbleGamepad` no controle 1
  (`host/gs/platform.rs: RUMBLE`). A opção de vibração do próprio jogo continua decidindo se vibra.
- Compilação release conferida na nuvem com todo o trabalho do Codex aplicado. Falta testar no Windows
  (janela GPU, controle físico).

## Primeiro grupo de skeletons removido do carregamento

96 tabelas de sincronização labial (9756 sequências) agora usam `lip-sync.json`,
lido pelo Rust sem skeleton nem arquivo original. A comparação dos 96 recursos
em pastas somente JSON passou com zero diferenças byte a byte. Expansão de
sequências, entradas vazias e 13 entradas inválidas também foram verificadas.
Builds release do compilador de recursos e do jogo passaram.

Restam 2007 skeletons ativos e 185 bancos: 2192 contêineres. Os templates antigos
foram preservados como referência offline. Detalhes, limites e inventário em
[native-lips.md](native-lips.md).

## Reuso dos recursos preparados em memória

A ponte PC agora reutiliza a mesma alocação e o mesmo identificador quando
recebe bytes idênticos novamente. A busca por hash confirma todos os bytes,
preservando recursos diferentes em caso de colisão. Identificadores antigos
continuam válidos; não foi introduzida expulsão de entradas ainda referenciadas.
Isso evita cópias retidas em recargas, mas não limita a memória de recursos
distintos nem estabelece um ganho medido para uma sessão completa.

Inventário repetido em `build/audits/resource-dedup/coverage.json`: 68601/68601
recursos representados, zero ausentes e 2288 contêineres ainda dependentes de
layouts binários. Isso mede cobertura de extração, não conclusão do port.

Validação: compilação release e dois testes passaram: recargas/colisões no
armazenamento e contrato de cópia para a memória do jogo, incluindo reuso do
identificador. Não foi repetida uma partida nem medida a memória de uma sessão.

## Textos e scripts sem templates binários; transporte PC sem recompressão

100 tabelas de texto (30000 entradas) e 50 scripts GSC agora são carregados
diretamente de JSON/texto, sem skeleton.bin. Os 150 recursos passaram pela
comparação byte a byte em pastas isoladas sem templates. Textos maiores
recalculam os offsets; scripts podem crescer além da capacidade antiga.
O compilador offline bt3-assetc compartilha os mesmos leitores do jogo.

Também foi removida a recompressão BPE dos recursos preparados no modo PC.
Imagens e entradas de animação passam por referências transitórias para os
dados já preparados na memória do host. Luta completa e abertura/título
passaram com decoder BPE legado bloqueado. Modelos/cenário foram conferidos
numa captura GPU. Essa ponte não elimina os layouts binários dos gráficos.

Restam 2103 representações skeleton e 185 bancos: 2288 contêineres.
Detalhes: docs/native-text-scripts.md e docs/pc-resource-transport.md.
O percentual global do port permanece sem medição confiável.


## Áudio migrado para recursos PC

361 bancos de efeitos agora usam audio-bank.json e WAVs, sem reconstrução de
bancos Sony, cópias IOP ou uploads SPU. As 18924 associações de samples foram
comparadas com o PCM anterior: zero diferenças. Música, vozes e áudio dos vídeos
usam o diretório nativo assets/audio/streams, com duração e loops preservados;
o modo PC não recorre ao ADX. Dois combates completos passaram com os caminhos
legados proibidos, incluindo 360 comandos de efeitos por combate.

O inventário de dependências ativas caiu para 2253 representações skeleton e
185 bancos de animação/recursos, totalizando 2438 contêineres ainda dependentes
de layouts de compatibilidade. O port completo continua sem percentual global
confiável. Detalhes: docs/native-audio.md.


## Recursos PC com fallback bloqueado

O `jogar.cmd` agora usa `BT3_PC_ONLY=1`: acesso aos arquivos originais do disco
é proibido. Abertura e encerramento usam Matroska sem perda, streams usam WAV
diretamente e os 601 recursos sem extração passaram a JSON estruturado.
Cobertura de representação: 68601/68601 recursos de arquivo. Uma luta demo
completa e boot com abertura completa passaram sem acesso ao raw.

Isso não significa eliminação de todos os formatos internos PS2: ainda há
skeleton.bin, bancos e reconstrução de layouts GS/VIF/SPU/BPE nos recursos
convertidos. O percentual global permanece sem medição confiável.
Detalhes e próximos requisitos: `docs/pc-independence.md`.


## Vídeos implementados

Abertura e encerramento agora usam `host/movie.rs`: decodificação FFmpeg,
imagem pelo caminho GS original, áudio pelo mixer Rust existente e pulo START
com o fade original. Ambos reproduzidos até EOF no Windows GPU (2810/4200
quadros); pulo e retorno ao título também observados. Ver `docs/movies.md`
para dependências, evidências e limites da verificação.

## Relato manual do usuário após a integração

Em 15/10/2026, o usuário informou ter jogado os fluxos de história, um torneio
completo, treinamento, loja, evolução e Ultimate Battle, além de testar saves,
sem encontrar bugs até então. Esse é um avanço de cobertura manual sobre os
registros técnicos anteriores; não implica prova de todas as combinações.
A prioridade seguinte solicitada explicitamente foi implementar os vídeos.

## Atualização após a revisão: Load Game integrado

O pacote `_nuvem_load_game` foi integrado nos sete caminhos revisados, com cópia
das versões anteriores em `build/audits/load-game-integration/before`.
Build release concluído e execução Windows GPU encerrada com código 0.
As capturas confirmam a lista, cancelamento, reabertura e chegada ao menu
principal após carregar um perfil. Foram usadas cópias de perfis de teste,
sem escrever nos saves reais. Ver `docs/saves.md` e os registros em
`build/audits/load-game-integration/runtime2`.

As seções abaixo preservam a análise feita antes dessa integração. O próximo
trabalho passa a ser a cobertura de saves/progressão dos modos e serviços PC
ainda ausentes (vídeos e controles), não reimplementar o Load Game.

## Base atual

Revisão dos arquivos locais, histórico Git e registros da execução Windows.
HEAD: `ecc467ef29f8c98bd3eae254c73974076e2b8e07`.
Não foi iniciada outra execução do jogo nem alterado um perfil de usuário nesta
revisão. Os resultados de execução abaixo são registros existentes, não novos
testes completos de todos os modos.

A base mais recente é **game**, crate independente `bt3-original`,
excluído do workspace principal. É uma tradução do C decompilado para Rust,
preservando endereços, layouts e aritmética PS2. Isso não equivale ainda a código
idiomático com estruturas de jogo totalmente independentes do modelo de memória
original. Há uma implementação de GS/VIF/VU e caminhos de shaders; os serviços
de PC ficam em `game/src/host`.

Entradas:

- `game/jogar.cmd`: jogo traduzido com janela GPU e controles.
- `game/demo.cmd`: demonstração nessa versão.
- `game/teste.cmd` e `vigia.cmd`: ferramentas de execução assistida;
  processam pedidos, compilam e podem alterar os registros existentes.
- `Previa.cmd`: runtime anterior de apresentação; não representa a versão
  traduzida mais recente.
- `play.ps1`: híbrido C/Rust anterior.

## Mudanças confirmadas no código e no Git

- Tradutor `tools/translate/c2rs`, módulos gerados em `src/tu` e ligações em
  `src/gen`. Alterações futuras de geração precisam ser feitas no tradutor,
  além de qualquer correção local necessária no código gerado.
- Aritmética e memória em `src/rt`.
- Renderer GPU OpenGL/SDL3 e shaders em `src/host/gs`.
- Áudio WASAPI/SDL e uso dos WAVs convertidos em `src/host/snd.rs`.
- 2.388 tabelas incorporadas; `src/gen/tables.rs` está presente localmente e é
  ignorado no Git por conter dados do jogo.
- Arquivos editáveis em `assets/files`: PNG, glTF, WAV, textos, eventos,
  câmeras, scripts, reconstruídos para os layouts esperados pelo jogo.
- Saves de PC em `src/host/saves.rs` e interface em `src/host/ui.rs`:
  três perfis, backup, salvamento automático e replays por perfil.

Limite dos assets: o carregador tenta a extração editável e mantém fallback
para `BT3_DATA`. `skeleton.bin` e os layouts originais continuam fazendo parte
da reconstrução. Não assumir que o jogo inteiro já foi distribuído e verificado
sem dados legados apenas porque o pacote antigo de abertura não os usava.

## Evidências encontradas

- `teste/verificar_log.txt`: **68.002 arquivos reconstruídos, zero diferentes**.
  Isso mede reconstrução byte a byte, não independência de todos os dados PS2.
- `teste/compilacao.txt`: build release concluído; avisos numerosos no código
  traduzido. Não usar `cargo fix` em massa para tentar eliminá-los.
- `teste/estado.txt` e `saida.txt`: execução de menus encerrada com código 0,
  limitada a 3.000 vblanks.
- `jogar_erros.txt`: renderer AMD Radeon RX 6750 XT, áudio WASAPI e registros
  de gravação em slot1/slot2. A presença de saves nos registros não prova todos
  os caminhos de carregamento e replay.
- Captura `teste/shots/gpu_02970.png` inspecionada: título original exibido.
- O histórico e `tools/difftest/README.md` documentam comparações da demo e
  estados de batalha contra C. Não foram repetidas nesta revisão.

## Ponto exato de retomada

O pacote **_nuvem_load_game** está não versionado e ainda não aplicado. Seu
`LEIAME.md` é a passagem de trabalho deixada pelo Claude. O projeto principal
ainda não tem `src/host/title.rs` nem as ligações desse pacote.

Ele acrescenta Load Game ao título, com cinco wrappers de funções e uma textura,
e muda o boot para carregar o perfil salvo mais recentemente. As diferenças de
`host/mod.rs`, `gen/funcs.rs`, `host/saves.rs` e `c2rs/link.py` foram examinadas.

Defeito conhecido relatado pelo Claude: confirmar Load Game inicia o fluxo 1,
mas a lista de perfis não aparece e o título não avança. Não considerar o pacote
pronto nem copiar arquivos em massa sem corrigir e observar esse caminho.

Sequência de continuidade:

1. Integrar as mudanças do pacote preservando a versão principal e a origem
   das ligações no tradutor.
2. Investigar `McFlow_Update -> picker_step -> draw` e o overlay GPU, bem como
   `host_buttons` durante a entrada modal. `Title_Run` já chama McFlow_Update
   a cada frame; não acrescentar um segundo avanço do fluxo por suposição.
3. Verificar seleção, cancelamento e retorno ao menu com perfis isolados.
   Não sobrescrever os saves reais para reproduzir o defeito.
4. Registrar o resultado concreto antes de tratar Load Game como concluído.

Outras pendências recebidas: saves de história/Opções/replays em situações
reais, vídeos PSS, vibração, cobertura dos modos, comportamento de Esc e,
posteriormente, online.

Sem porcentagem global confiável. A tradução de todas as funções não comprova
que todos os recursos estejam integrados e corretos no PC.

## 2026-10-18: 16:9, opções gráficas no menu do jogo, texturas HD

* **16:9 por padrão** (`host/settings.rs` aspect; também 4:3, 16:10, 21:9).
  * `Port_WideFactor` alarga a projeção, o recorte do cenário e as proporções dos efeitos.
  * O renderizador estreita a arte 2D em torno de pontos fixos: as bordas e o meio da interface da luta, pelos
    marcadores do código da interface.
  * São as regras do port em C, agora portadas para o Rust.
  * Os menus, compostos para 4:3, aparecem no meio sobre uma cópia desfocada e escurecida de si mesmos. Esticados,
    deformavam os personagens; abertos para os lados, mostravam faixas pretas e arte cortada.
* **Options > Graphics** (`host/options.rs`), na tela de opções do próprio jogo:
  * uma quinta linha na página principal, desenhada como uma cópia de uma placa do filme (como o "Load Game" da
    tela inicial);
  * uma página feita a partir do filme da página Screen, com resolução, proporção, qualidade das texturas e tela
    cheia, mais Default;
  * textos no estilo do jogo, gerados por `tools/assets/make_option_labels.py`, inclusive o título da página na
    janela do ícone;
  * tudo vale na hora: a resolução refaz os buffers do renderizador e ajusta o tamanho da janela.
* **Tela inicial:** o item **Options** (`host/title.rs`, agora com quatro itens) abre a mesma tela por cima da
  tela inicial e volta para ela. Aberta dali, a tela não roda o fluxo de salvar do jogo, porque o perfil pode ainda
  não estar carregado.
* **F1:** ficou só com extras do PC (as teclas do controle no teclado).
* **Texturas HD** (`host/texpack.rs`, [texture-packs.md](texture-packs.md)):
  * o pacote do canal PLAYANDO para o Dolphin (Wii), encontrado pelos hashes do Dolphin;
  * as duas metades cinza das texturas com transparência são juntadas numa imagem colorida;
  * qualidades Low, Medium, High e Ultra;
  * o instalador baixa os pacotes ou usa um `.rar` ou uma pasta que a pessoa já tenha (`tools/install/texturas.py`).
* **Verificado na nuvem** (renderizador headless):
  * tela inicial, opções e luta de demonstração em 16:9;
  * página Graphics, com a troca de resolução e de proporção no meio do jogo;
  * texturas HD na tela inicial e na luta (logo, personagens, céu e rochas com as cores certas).
* **Ainda não verificado:**
  * o download pelo Google Drive e a extração do RAR no Windows;
  * a janela em 16:9 com o fundo desfocado dos menus (os testes não têm janela);
  * a interface da luta nas bordas.

## 2026-10-18: instalação mais rápida

* **Passo 4** (`tools/assets/files_extract.py`): converte os arquivos em paralelo, um processo por núcleo
  (`BT3_EXTRACT_JOBS` muda o número).
  * Os bancos de animação ficam para uma segunda rodada, porque precisam do modelo do lutador já escrito.
  * A lista de arquivos prontos continua permitindo parar e retomar.
  * A saída é idêntica à da versão em série: 106 arquivos de teste, 2.682 arquivos escritos, sem nenhuma diferença.
  * Com 2 núcleos, o tempo caiu de 52 s para 27 s.
* **Passo 3** (`bt3-install export`): a cópia e a conversão de músicas e vozes em WAV rodam em várias threads.
  O catálogo é escrito na ordem de antes e a saída é idêntica.
* **Empacotamento** (`tools/install/pack.py`): a compressão roda em várias threads. O pacote sai byte a byte igual.
* **Instalador:** a barra do passo 4 usa o total real de arquivos (cerca de 9.500 com as quatro partições), em vez
  de 3.400.
