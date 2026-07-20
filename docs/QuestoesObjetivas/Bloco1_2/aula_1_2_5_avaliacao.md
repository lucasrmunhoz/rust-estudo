# Aula 1.2.5 — Avaliação

**Conversão de Tipos: `as`, `From`/`Into`, `TryFrom`/`TryInto`, `div_euclid`/`rem_euclid`**

> Dez questões objetivas: cinco Verdadeiro ou Falso, cinco de múltipla escolha (a–e, uma única alternativa correta).
> Preencha os espaços reservados com sua resposta.

---

## Parte A — Verdadeiro ou Falso

**1.** O worldgen do VNP está gerando o terreno do octante negativo. Para a coordenada de mundo `x = -33` e chunks de lado 32, as chamadas `x.div_euclid(32)` e `x.rem_euclid(32)` devolvem, respectivamente, `-2` e `31` — ou seja, o voxel é a última célula do chunk `-2`.

**Resposta:** V

---

**2.** Durante o processamento de um save antigo, o código executa `let id: u8 = valor_u32 as u8;` com `valor_u32 = 300`. Como o `Cargo.toml` do projeto define `overflow-checks = true` no perfil dev, essa conversão causa panic em modo debug, do mesmo modo que uma soma que estoura o tipo causaria.

**Resposta:** F

---

**3.** No lookup da paleta de blocos, as formas `usize::from(id)` e `id as usize` (com `id: u8`) compilam para o mesmo código de máquina; a diferença entre elas não está no custo em runtime, mas na garantia que o compilador fiscaliza em tempo de compilação.

**Resposta:** V

---

**4.** Após executar `let local_x = mundo_x.rem_euclid(32);`, a variável `mundo_x` passa a conter o valor local no intervalo `0..32`, pois `rem_euclid` ajusta o valor sobre o qual é chamado; por isso o resultado pode ser usado diretamente dela.

**Resposta:** F

---

**5.** A física da câmera produziu um `f32` com valor `NaN`, e esse valor atravessa `as i32` para um snap de coordenada. Em modo debug, essa conversão causa panic, interrompendo o programa antes que o `NaN` contamine as coordenadas inteiras.

**Resposta:** V

---

## Parte B — Múltipla escolha

**6.** Um colega escreveu o acesso a voxel sem a etapa euclidiana: `let i = mundo_x as usize; let id = voxels[i];`, com `mundo_x: i32 = -1` e `voxels` de comprimento 32.768, rodando no Linux 64-bit. O que acontece, e por quê?

a) Em modo release, `i` vira `18_446_744_073_709_551_615` e o acesso lê silenciosamente memória adjacente ao vetor, corrompendo dados — os bounds checks só existem em debug.

b) A conversão satura: como `-1` está abaixo do mínimo de `usize`, o resultado é `0`, e o código lê o voxel errado (o primeiro do chunk) sem qualquer aviso.

c) O código não compila: o compilador rejeita `as usize` sobre um `i32` que pode ser negativo, exigindo `TryFrom` para essa travessia.

d) A conversão causa panic em modo debug por causa do `overflow-checks = true`, que detecta a mudança de sinal; em release, o valor gigante passa e o índice explode.

e) `i` vira `18_446_744_073_709_551_615` por extensão de sinal seguida de reinterpretação; o acesso `voxels[i]` causa panic de *index out of bounds* — tanto em debug quanto em release, pois a verificação de limites da indexação não é desligada pelo perfil.

**Resposta:** e

---

**7.** O jogador está em `x ≈ 4_200_000`. O código de render computa `let gpu_x = (voxel_x as f32) - (cam_x as f32);` e a geometria treme visivelmente. Qual é o diagnóstico correto e a correção?

a) Na faixa de ~4,2 milhões, o passo entre `f32` vizinhos é `0,5`: cada operando chega à subtração já arredondado para essa grade, e o delta carrega erro de até frações de voxel. Correção: subtrair primeiro em `i32` (exato) e converter só o delta pequeno.

b) A subtração em `f32` acumula erro de arredondamento a cada frame renderizado; depois de alguns segundos o erro acumulado passa do tamanho de um voxel. Correção: zerar o acumulador periodicamente reconvertendo a partir dos inteiros.

c) Acima de 2²² ≈ 4,2 milhões o `f32` não representa mais nenhum inteiro e ambos os casts saturam para o máximo representável; a subtração devolve zero. Correção: usar `f64` nos casts e truncar no final.

d) `i32 as f32` opera por reinterpretação dos bits em complemento de dois, como no caso signed↔unsigned; para valores grandes o padrão de bits relido produz floats sem relação com o original. Correção: converter via `u32` intermediário.

e) `i32 as f32` trunca em direção a zero, como a conversão float→int; a parte "fracionária" da coordenada é descartada nos dois operandos. Correção: somar `0.5` antes de cada cast para compensar o truncamento.

**Resposta:** a

---

**8.** No cálculo do índice plano, um colega tenta `let i: usize = usize::from(coluna_u32);` e recebe erro de compilação. Ele argumenta: "mas `u32` sempre cabe em `usize` nos nossos alvos, Linux e Windows 64-bit". Qual é a explicação correta da recusa?

a) É uma lacuna conhecida da biblioteca padrão: a implementação existe para `u8` e `u16` e a de `u32` ainda não foi adicionada; enquanto isso, `as` pode ser usado livremente e sem comentário nesse ponto.

b) O erro ocorre porque `From` não decide entre os tamanhos de `usize` do Windows (32 bits) e do Linux (64 bits) no mesmo binário; como o projeto compila para os dois sistemas, o compilador recusa por ambiguidade de alvo.

c) `From` para `usize` existe apenas para tipos signed, pois índices podem nascer de deltas negativos já validados; a forma correta seria converter `coluna_u32` para `i32` antes e então usar `usize::from`.

d) `From` só é implementado onde a preservação do valor é garantida em **qualquer** alvo de compilação concebível, e a linguagem não garante que `usize` tenha ao menos 32 bits; a ponte `u32 → usize` exige `as` com justificativa de domínio ou `TryFrom` com tratamento de falha.

e) O compilador está protegendo contra valores negativos; como `u32` é unsigned e não pode ser negativo, basta acrescentar a anotação de tipo no binding (`let i: usize`) que a inferência resolve e o `from` compila.

**Resposta:** d

---

**9.** Um teste do mapeamento mundo→chunk precisa validar a coordenada `mundo_x = -70` com lado 32. Qual par (chunk, local) o teste deve esperar?

a) chunk `-2`, local `-6` — o resultado direto de `/` e `%`, que são os operadores nativos da linguagem e portanto a referência de correção do teste.

b) chunk `-2`, local `26` — o quociente trunca em direção a zero, mas o resto euclidiano corrige o deslocamento de forma independente do quociente.

c) chunk `-3`, local `26` — o quociente arredonda em direção a −∞ e o resto mede a distância da borda esquerda do bloco `[-96, -64)`; a identidade `-3 × 32 + 26 = -70` confirma o par.

d) chunk `-3`, local `6` — em chunks negativos o eixo local conta a partir da borda mais próxima do zero, espelhando a contagem dos chunks positivos.

e) chunk `-3`, local `-26` — o resto euclidiano preserva o sinal do dividendo para que a identidade `q × 32 + r = a` continue válida com quociente arredondado para baixo.

**Resposta:** c

---

**10.** Em revisão de código, você encontra `let props = &paleta[id as usize];`, com `id: u8` recém-lido do array de voxels. Pela convenção do projeto — usar sempre a ferramenta com a garantia mais forte disponível — qual é o veredito?

a) O código está correto como está: para `u8 → usize` as duas formas geram o mesmo código de máquina, logo a convenção não tem preferência entre elas neste ponto.

b) Deve virar `usize::from(id)` (ou `id.into()`): a conversão com garantia do compilador existe para `u8 → usize`, e a convenção reserva o `as` para as travessias onde nenhuma garantia existe, como `i32 → usize`.

c) Deve virar `usize::try_from(id)` com tratamento de falha: em plataformas de 16 bits um `u8` pode não caber em `usize`, e só `TryFrom` cobre esse cenário de portabilidade.

d) Deve virar `usize::from(id)` porque `id as usize` opera por reinterpretação com extensão de sinal e pode produzir o índice gigante quando o bit mais alto do `u8` está ligado.

e) O cast é desnecessário: como `u8` é unsigned e a paleta tem no máximo 256 entradas, `paleta[id]` compila diretamente — a indexação aceita qualquer inteiro sem sinal.

**Resposta:** b

---

## Gabarito de preenchimento rápido

| Questão | Resposta |
|:---:|:---:|
| 1  |  |
| 2  |  |
| 3  |  |
| 4  |  |
| 5  |  |
| 6  |  |
| 7  |  |
| 8  |  |
| 9  |  |
| 10 |  |
