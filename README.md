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
3. escolha a pasta (padrão: `C:\Program Files (x86)\Tenkaichi 3`) e clique em **Instalar**.

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
7. instala o jogo: os dados empacotados (Jogar) ou como arquivos soltos (Desenvolvedor).

**Controles e opções:**
* **Teclado:**
  * setas = direcional; W A S D = analógico;
  * K = X, L = ○, J = □, I = △;
  * U = L1, O = R1, 7 = L2, 9 = R2;
  * Enter = Start, Shift direito = Select.
* **Controles:** Xbox e PlayStation, pelo SDL3, com vibração ([docs/controllers.md](docs/controllers.md)).
* **Configurações:** F1 abre as configurações (tela cheia, resolução interna 1x a 4x, teclas), guardadas em
  `Documentos\My Games\Tenkaichi3Rust\settings.ini`. F11 alterna a tela cheia. Esc pergunta se quer sair.
* **Saves:** 3 perfis em `Documentos\My Games\Tenkaichi3Rust\saves`, com salvamento automático e "Load Game" na
  tela de título ([docs/saves.md](docs/saves.md)). Se o jogo parar por um erro, o motivo fica em
  `Documentos\My Games\Tenkaichi3Rust\log.txt`.

O jogo roda só com os arquivos extraídos: nenhum arquivo original do disco é lido durante o jogo.

## Gerar o instalador

No Windows, com [Rust](https://rustup.rs) e [Python 3](https://www.python.org): `python tools/install/empacotar.py`
gera `dist/Instalar Tenkaichi 3.exe` (o programa da janela, com o Python embutido, as ferramentas de extração e o
jogo compilado dentro).

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

1. Instalador com dois modos:
   * **jogador:** executável pronto e dados empacotados, sem Python nem Rust;
   * **desenvolvedor:** o que existe hoje.
2. Dar nomes e estrutura aos ~6,6 MB restantes.
3. Jogo online.

## Licença

MIT ([LICENSE](LICENSE)). Os dados do jogo pertencem aos seus donos e não fazem parte do projeto.
