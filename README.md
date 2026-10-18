# Tenkaichi3Rust

Dragon Ball Z: Budokai Tenkaichi 3 (PS2, EUA, SLUS-21678) como um jogo de PC escrito em Rust.

O código do jogo é a tradução mecânica de todo o C da decompilação: 7462 funções, idênticas ao original quadro a
quadro nos testes. Tudo o que o PS2 fazia por fora do jogo foi escrito para o PC: imagem pela placa de vídeo
(OpenGL), som, controles, saves, vídeos e arquivos em formatos abertos.

O repositório não traz nada do jogo. Todos os dados vêm do disco de quem instala.

## Instalar e jogar

Baixe **`Instalar Tenkaichi 3.exe`** (na página de versões do projeto) e abra. Na janela:

1. escolha a sua imagem do disco (`.iso`);
2. escolha o tipo de instalação:
   * **Jogar:** o jogo pronto, com os dados em poucos pacotes, como um jogo de PC;
   * **Desenvolvedor:** o mesmo jogo, com todos os arquivos do jogo editáveis (PNG, glTF, WAV, JSON), para mods;
3. escolha a pasta (padrão: `C:\Program Files (x86)\Tenkaichi 3`);
4. se quiser, marque as **texturas HD** (1080p e/ou 4K: veja abaixo) e clique em **Instalar**.

A barra de progresso mostra cada passo. A primeira instalação demora bastante, e pode ser cancelada e retomada:
os passos prontos são pulados. Os dois tipos criam `Tenkaichi3.exe` e atalhos na área de trabalho e no menu
Iniciar, e o jogo aparece em Configurações > Aplicativos, de onde se desinstala (ou pelo `Desinstalar.exe` da
pasta do jogo, que pergunta se apaga também os saves). Não é preciso instalar Python, Rust nem FFmpeg. O instalador não traz nada do jogo: tudo vem da sua imagem
do disco.

O que o instalador faz:

1. lê o disco, sem 7-Zip;
2. escreve as tabelas do jogo (`assets/data/tables.txt`), que o executável lê ao abrir;
3. converte músicas e vozes em WAV;
4. converte todos os arquivos para formatos abertos (PNG, glTF, WAV, JSON, texto);
5. prepara os recursos de PC e organiza as pastas por tipo e nome;
6. separa o vídeo MPEG-2 dos filmes, sem converter (o jogo o decodifica em Rust);
7. instala o jogo: os dados empacotados (Jogar) ou como arquivos soltos (Desenvolvedor);
8. se marcadas, baixa e extrai as texturas HD.

**Imagem:** o jogo abre em **16:9** por padrão. Nas lutas a câmera mostra mais dos lados, sem esticar nada, e a
interface fica nas bordas. Os menus, desenhados para 4:3, aparecem no meio sobre uma cópia desfocada de si mesmos.

**Texturas HD:** o remaster de texturas do canal **PLAYANDO**
([vídeo](https://www.youtube.com/watch?v=rJW7idQDqVc)), feito para a versão de Wii no Dolphin, funciona aqui: as
texturas do Wii são as mesmas do PS2, índice por índice, então o jogo acha cada arquivo do pacote pelo nome que o
Dolphin dá. O instalador baixa os pacotes dos links do autor no Google Drive (1080p: 1 GB, 4K: 3,4 GB) ou usa um
`.rar` ou uma pasta que você já tenha. Eles ficam em `assets\textures\2x` e `assets\textures\4x` da pasta do jogo.
Os pacotes não fazem parte deste projeto ([docs/texture-packs.md](docs/texture-packs.md)).

**Controles e opções:**
* **Teclado:**
  * setas = direcional; W A S D = analógico;
  * K = X, L = ○, J = □, I = △;
  * U = L1, O = R1, 7 = L2, 9 = R2;
  * Enter = Start, Shift direito = Select.
* **Controles:** Xbox e PlayStation, pelo SDL3, com vibração ([docs/controllers.md](docs/controllers.md)).
* **Opções:** a tela de opções do próprio jogo (também pela tela inicial, em **Options**) ganhou a página
  **Graphics**: resolução (720p, 1080p, 1440p, 4K), proporção (4:3, 16:10, 16:9, 21:9), qualidade das texturas
  (Low = as do PS2, Medium, High = pacote 1080p, Ultra = pacote 4K) e tela cheia. Tudo vale na hora e fica em
  `Documentos\My Games\Tenkaichi3Rust\settings.ini`.
* **Extras do PC:** F1 (teclas do controle no teclado). F11 alterna a tela cheia. Esc pergunta se quer sair.
* **Saves:** 3 perfis em `Documentos\My Games\Tenkaichi3Rust\saves`, com salvamento automático e "Load Game" na
  tela de título ([docs/saves.md](docs/saves.md)). Se o jogo parar por um erro, o motivo fica em
  `Documentos\My Games\Tenkaichi3Rust\log.txt`.

O jogo roda só com os arquivos extraídos: nenhum arquivo original do disco é lido durante o jogo.

## Gerar o instalador

No Windows, com [Rust](https://rustup.rs) e [Python 3](https://www.python.org): `python tools/install/empacotar.py`
gera `dist/Instalar Tenkaichi 3.exe` (o programa da janela, com o Python embutido, as ferramentas de extração e o
jogo compilado dentro).

* `tools\install\testar_build.cmd` testa a build antes: compila o jogo, joga a tela inicial e uma luta numa janela
  escondida e confere se a imagem da janela está certa (precisa de uma instalação de desenvolvedor em `assets\files`
  do projeto; as imagens ficam em `dist\teste`).
* `tools\install\gerar_instalador.cmd` faz as duas coisas em segundo plano (janela minimizada): testa e, se passar,
  gera o instalador. O andamento fica em `dist\gerar.log`.

Atualizar: o instalador deixa `assets\instalacao.txt` com a versão dos dados (um hash das ferramentas que os
fazem). Instalando de novo na mesma pasta, com os mesmos dados e o mesmo tipo, ele só troca os programas, em
segundos e sem a imagem do disco. Se as ferramentas mudaram, refaz tudo; a caixa "Refazer tudo do zero" força isso.

## Modificar

Na instalação de desenvolvedor, os arquivos do jogo ficam em `assets\files` da pasta do jogo, agrupados por tipo e
com nome:

```
assets/files/
  lutadores/Goku (Early)/      01423 roupa 1/, 01427 roupa 1 (danificada)/, 01431 animações/,
                               01432 efeitos das técnicas/, 03033 vozes/, 00760 retrato/ ...
  cenários/12 Kame House/      00380 cenário/, 00345 sons do cenário/, 00936 imagem de seleção/ ...
  menus/  sons/  textos/  roteiros/  modelos/  imagens/  sincronia labial/  ...
```

Dentro de cada pasta estão os arquivos editáveis:

* texturas em PNG;
* modelos, cenários, colisão e animações em glTF;
* sons em WAV;
* textos, tabelas e parâmetros em JSON ou texto.

Ao carregar um arquivo, o jogo o monta a partir desses arquivos. Edite-os e abra `Tenkaichi3.exe`. O número no começo de cada pasta
é como o jogo acha o arquivo: ele precisa ficar, mas o resto do nome e a pasta onde ela está podem mudar. Os formatos
estão descritos em [docs/open-formats.md](docs/open-formats.md).

## Organização

```
game/              o jogo: crate bt3-original (src/tu = código traduzido, src/host = serviços de PC)
tools/install/     instalador com janela (src/bin/instalador.rs) e os passos (instalar.py)
tools/assets/      extração e conversores para formatos abertos (Python)
tools/translate/   tradutor C → Rust que gera game/src
tools/difftest/    testes diferenciais contra o C da decompilação
vendor/windows/    bibliotecas de terceiros (SDL3)
docs/              documentação
```

Os detalhes estão em [docs/project-layout.md](docs/project-layout.md). Para contribuir, veja
[CONTRIBUTING.md](CONTRIBUTING.md).

## Estado atual

* Todos os modos foram jogados sem defeitos conhecidos: história, torneio, treino, loja, evolução, Ultimate Battle e
  duelos.
* Todo arquivo do disco é refeito a partir dos formatos abertos sem nenhuma diferença.
* A luta de demonstração só com recursos de PC é a mesma, quadro a quadro, da que usa os arquivos originais.
* Cerca de 6,6 MB ainda não têm estrutura conhecida e ficam como listas de palavras de 32 bits, no lugar exato
  (`file-tables.json`).

Histórico do projeto: [docs/history.md](docs/history.md).

## Próximas etapas

1. Dar nomes e estrutura aos ~6,6 MB restantes.
2. Cheats e outros extras no F1.
3. Jogo online.

## Licença

MIT ([LICENSE](LICENSE)). Os dados do jogo pertencem aos seus donos e não fazem parte do projeto.
