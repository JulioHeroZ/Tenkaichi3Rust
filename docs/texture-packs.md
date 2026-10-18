# Texturas HD (pacotes do Dolphin)

O jogo usa o remaster de texturas do canal **PLAYANDO**, apresentado no vídeo
[REMASTERIZEI SOZINHO O DRAGON BALL Z BUDOKAI TENKAICHI 3](https://www.youtube.com/watch?v=rJW7idQDqVc). Esse
pacote foi feito para a versão de Wii (código RDSE70) rodando no Dolphin e é distribuído de graça pelo autor no
Google Drive:

* `TEXTURAS 1080p.rar`: 1 GB, as texturas com 2x o tamanho original;
* `TEXTURAS 4K.rar`: 3,4 GB, com 4x.

Os pacotes não fazem parte do projeto e nem do instalador. O instalador baixa os arquivos dos links do autor
(`tools/install/texturas.py`), ou usa um `.rar` ou uma pasta que a pessoa já tenha, e os põe em
`assets/textures/2x` e `assets/textures/4x`. Para abrir o RAR, ele usa o que o computador tiver: o `tar` do Windows
11, o 7-Zip ou o WinRAR.

## Como o jogo acha cada arquivo

O jogo de Wii usa as mesmas texturas do PS2, índice por índice. O Dolphin nomeia cada arquivo assim:
`tex1_<largura>x<altura>_<hash da textura>_<hash da paleta>_<formato>.dds`. Os dois hashes são calculados assim
(`VideoCommon/TextureInfo.cpp` do Dolphin):

* **Hash da textura:** XXH64 (seed 0) dos índices no layout em blocos do Wii. São blocos de 8x8 para índices de
  4 bits (dois por byte, o primeiro no nibble alto) e de 8x4 para índices de 8 bits.
* **Hash da paleta:** XXH64 das cores da paleta, do menor ao maior índice usado, cada uma em RGB5A3 big-endian.
* **Formato:** 8 (C4) ou 9 (C8).

Quando a textura não está no cache, o `texture_get` do renderizador (`game/src/host/gs/gpu.rs`) lê os índices da
memória do GS, calcula o hash e procura o arquivo (`game/src/host/texpack.rs`). Tudo isso acontece com as texturas
do jogo enquanto ele roda, sem nenhuma conversão do pacote.

## O que os arquivos contêm

* **Paleta opaca:** uma imagem colorida. O hash da paleta bate exatamente com a paleta do PS2 convertida para
  RGB5A3.
* **Paleta com transparência:** o Wii não tem paleta com 8 bits de cada canal mais alpha. Então a mesma textura é
  lida com duas paletas de intensidade e alpha (IA8): uma dá o vermelho (intensidade) e o verde (alpha), a outra dá
  o azul e o alpha do PS2 (0x80 = opaco). O pacote traz as duas imagens, cinza, com dois hashes de paleta. O jogo
  descobre qual é qual comparando com a textura original (um resumo de 8x8 células) e junta as duas numa imagem
  colorida.
* **Texturas pintadas pelo jogo com várias paletas:** vários arquivos. Quando o hash da paleta não bate com nenhum,
  vale o arquivo (ou o par de metades) cujo resumo fica mais perto do original. A escolha é guardada por textura e
  paleta.
* **Alpha:** o de uma imagem do Wii vai até 255 (255 = opaco). Arquivos cujo alpha não passa de 130 já trazem o
  alpha do PS2. O shader recebe a escala certa em `orig.w`.

Os arquivos são lidos e decodificados (DXT1, DXT3, DXT5 ou 32 bits) em duas threads. Até a substituta ficar
pronta, aparece a original. As substitutas vão para a placa de vídeo com mip levels, usados no 3D.

A arte 2D segue as regras do port em C (`gs_draw.c`):

* é amostrada por pixel de saída, limitada ao retângulo da folha que a peça mostra;
* sempre com filtro;
* sem mip levels;
* com o alpha limitado ao maior alpha da original.

## Qualidade das texturas (Options > Graphics)

| Opção | Pacote |
| --- | --- |
| Low | as texturas do PS2 |
| Medium | o pacote 1080p reduzido ao tamanho original (a arte redesenhada, sem ampliar) |
| High | o pacote 1080p (ou o 4K reduzido pela metade) |
| Ultra | o pacote 4K (ou o 1080p) |

Trocar a qualidade refaz as texturas na hora. `BT3_TEXTURES=<pasta>` acrescenta um lugar onde procurar os pacotes.
`BT3_TEX_LOG=1` mostra as escolhas entre arquivos.

## Limites conhecidos

* Texturas de 32 bits (formato 6 do pacote, cerca de 400 arquivos) ainda não são procuradas.
* Em algumas texturas de cenário, a escolha pelo resumo pode errar entre as paletas. Aparece como um detalhe
  diferente do original, por exemplo um arbusto no lugar de uma árvore seca.
