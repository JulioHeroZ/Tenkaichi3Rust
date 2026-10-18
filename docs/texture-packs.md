# Texturas HD (pacotes do PCSX2)

O jogo usa pacotes de texturas feitos para o **PCSX2** (a substituição de texturas do emulador). O pacote que o
instalador oferece é compartilhado no MediaFire:
[Dragon Ball Z -Budokai Tenkaichi 3.rar](https://www.mediafire.com/file/2jm75b9v163tieg/Dragon_Ball_Z_-Budokai_Tenkaichi_3.rar/file)
(2 GB). Ele traz as pastas `replacements/` (as texturas), `Fonts/` (letra nova ou original), `Original Buttons/` e
`Xbox Layout/` (dois desenhos dos botões).

O pacote não faz parte do projeto e nem do instalador. O instalador baixa o arquivo do link acima
(`tools/install/texturas.py`), ou usa o `.rar` ou uma pasta que a pessoa já tenha, e põe a pasta `replacements`
em `assets/textures/pcsx2`. Para abrir o RAR, ele usa o que o computador tiver: o `tar` do Windows 11, o 7-Zip ou o
WinRAR. As variantes de letra e de botões entram copiando a pasta delas por cima de `assets/textures/pcsx2`, como o
`Fonts/Installation.txt` do pacote explica.

Até a versão 0.2.x o jogo usava o remaster do canal PLAYANDO, feito para o Wii no Dolphin. O de PCSX2 é calculado
direto sobre os dados do PS2, então não há conversão de paletas nem escolha por semelhança: cada arquivo é a textura
exata. O instalador apaga o pacote antigo (`assets/textures/2x` e `4x`).

## Como o jogo acha cada arquivo

Do mesmo jeito que o port em C (Tenkaichi3Decomp, `port/src/gs/gs_texpack.c`). Cada arquivo se chama
`<hash da textura>-<hash da paleta>-<bits>.dds` (sem o hash da paleta numa textura sem paleta), em hexadecimal:

* **Hash da textura:** XXH3 de 64 bits (`game/src/host/xxh3.rs`) dos blocos de 256 bytes da memória do GS que a
  textura ocupa, bloco a bloco, linha a linha: os dados como estão na memória.
* **Hash da paleta:** XXH3 das cores de 32 bits da paleta, em ordem de índice, do menor ao maior índice que a
  textura usa (alguns arquivos usam a paleta inteira: o jogo procura os dois).
* **Bits:** `PSM | log2(largura) << 6 | log2(altura) << 10 | TCC << 14`.

Quando a textura não está no cache, o `texture_get` do renderizador (`game/src/host/gs/gpu.rs`) lê os blocos da
memória do GS e procura o arquivo (`game/src/host/texpack.rs`). Formatos: 8 bits e 4 bits com paleta, e 32 bits.

O alpha dos arquivos é o do GS (0x80 = opaco), igual ao das texturas que eles trocam. Os arquivos (DDS DXT1, DXT3,
DXT5 ou 32 bits, ou PNG) são lidos e decodificados em duas threads. Até a substituta ficar pronta, aparece a
original. As substitutas vão para a placa de vídeo com mip levels, usados no 3D. A arte 2D segue as regras do port
em C (`gs_draw.c`): amostrada por pixel de saída dentro do retângulo da folha, sempre com filtro, com o alpha
limitado ao maior alpha da original.

## Qualidade das texturas (Options > Graphics)

| Opção | Pacote |
| --- | --- |
| Low | as texturas do PS2 |
| Medium | o pacote com metade do tamanho |
| High, Ultra | o pacote como ele é |

Trocar a qualidade refaz as texturas na hora. `BT3_TEXTURES=<pasta>` acrescenta um lugar onde procurar. `BT3_TEX_LOG=1`
mostra cada textura com paleta, achada ou não, pelo nome que o arquivo teria.

## Limites conhecidos

* Texturas menores que um bloco do GS (16x16 em 8 bits, 32x16 em 4 bits, 8x8 em 32 bits) não são procuradas: o
  PCSX2 calcula outro hash para elas, que ainda não foi descoberto.
* Paletas de 16 bits não são procuradas (o jogo não usa).
