# Becos sem saída da tradução e do teste diferencial

O que já foi tentado e não funcionou, para não repetir. Uma linha por item: o que, por que falhou, data.

- `clang -m32 -msoft-float` para a referência i386: no x86 o clang ignora a opção e gera x87; diferenças de
  1 ulp em tudo. Usar `-mno-x87 -mno-sse` (toda conta float vira chamada às rotinas do PS2). 2026-10-14
- `-fdump-record-layouts-complete` do clang como fonte dos layouts: calcula cedo demais e ignora
  `aligned(16)` em typedefs (vetores); structs com vetores ficaram com offsets errados. Usar
  `make_layouts.py` (sizeof forçado + dump normal). 2026-10-14
- Reescrever na memória os ponteiros de função (palavras iguais a um ID de função) ao copiar o estado
  para a referência: palavras comuns coincidem com IDs e a referência quebra. Usar `fidify.py`
  (no IR, endereço de função = ID). 2026-10-14
- Comparar com a referência sem `-ftrivial-auto-var-init=zero` e sem zerar os frames no Rust: o lixo da
  pilha aparece como diferença em funções que leem campos nunca escritos (ex.: `offset.w` em
  `EftAura_SpawnFlames`). Hoje os dois lados zeram. 2026-10-14
- Perfil release com `incremental = true` e o ThinLTO local padrão: depois de alguns builds aparece
  `undefined hidden symbol anon.*` no link. Usar `lto = "off"`. 2026-10-14
- Contador de chamadas das sondas num `HashMap` por nome: a sonda é chamada em toda função e o jogo fica
  muito lento. Usar um `rt::ProbeCount` estático por função. 2026-10-14
- Dividir o crate gerado em vários crates para compilar mais rápido: as unidades se chamam em ciclo e
  crates do Rust não podem ter dependência cíclica; só daria com `extern "Rust"` + `#[no_mangle]` em
  tudo, e o build incremental (1 a 5 min para correções localizadas) já resolve o caso comum. 2026-10-14
- Skills e MCPs genéricos de engenharia reversa (REA, GhidraMCP, reverse-skill...): servem para obter C a
  partir do binário; aqui o C já existe e a verificação é o teste diferencial. 2026-10-14
- `pkill -f <padrão>` com um padrão que aparece na própria linha de comando mata o shell da sessão;
  usar `pkill -x <nome>` ou um padrão ancorado em `^python3 ...`. 2026-10-14
