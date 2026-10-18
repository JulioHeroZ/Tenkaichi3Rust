# Controles Xbox e PlayStation

O backend usa SDL3 Gamepad para normalizar os controles Xbox, DualShock 4 e
DualSense. Os botões seguem a posição física e são convertidos para as ações
originais do jogo. Os dois jogadores têm seleção de dispositivo e bindings
independentes. O teclado continua disponível.

| Xbox | PlayStation | Entrada do jogo |
|---|---|---|
| A | Cruz | Cruz |
| B | Círculo | Círculo |
| X | Quadrado | Quadrado |
| Y | Triângulo | Triângulo |
| LB / RB | L1 / R1 | L1 / R1 |
| LT / RT | L2 / R2 | L2 / R2 |
| Menu | Options | START |
| View | Share / Create | SELECT |
| Clique nos analógicos | L3 / R3 | L3 / R3 |

Direcional e os dois analógicos são lidos pelo mesmo backend (`game/src/host`, via SDL3), com vibração
(`SDL_RumbleGamepad`). F1 abre as configurações do PC, onde se troca as teclas do teclado
(`Documentos\My Games\Tenkaichi3Rust\settings.ini`).

O Windows precisa reconhecer o dispositivo, seja por USB ou após pareamento Bluetooth. Não há implementação própria
de recursos especiais como touchpad, gatilhos adaptativos ou áudio do controle.
