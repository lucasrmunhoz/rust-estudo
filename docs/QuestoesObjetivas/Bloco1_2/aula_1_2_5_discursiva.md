# **Aula 1.2.5 — Atividade Prática (questão discursiva)**

## **Cenário**

O jogador do VNP está explorando o octante negativo do mundo e quebra um voxel na coordenada de mundo `(-70, 5, -1)`. Esse único evento dispara a travessia completa das três pontes de conversão da engine: a coordenada de mundo precisa virar **chunk \+ local** (Ponte 1), o local precisa virar **índice plano** para achar o voxel no armazenamento (Ponte 2), e a posição precisa virar **vértice `f32`** para a GPU redesenhar a região (Ponte 3). Além disso, o ID do voxel quebrado consulta a **paleta de blocos** para saber a dureza do material. Você vai escrever esse trajeto inteiro, incluindo uma demonstração documentada dos dois caminhos errados — o operador truncado da Aula 1.2.4 e a subtração tardia em `f32` — para que o contraste fique gravado no próprio código.

## **Objetivo**

Aplicar, num único programa contínuo: `div_euclid`/`rem_euclid` (Ponte 1), `as` com justificativa de domínio (Pontes 1 e 2), `From` onde a garantia existe (paleta), a ordem subtração-antes-do-cast (Ponte 3\) — sustentados pelos fundamentos das aulas anteriores: tipagem explícita (1.2.2), bools derivados (1.2.3), operadores, identidade aritmética e short-circuit (1.2.4).

## **O que entregar**

O binário `src/bin/bloco_1_2/aula_1_2_5.rs`, com entrada `[[bin]]` registrada no `Cargo.toml` (nome `aula_1_2_5`), compilando **sem warnings** via `cargo run --bin aula_1_2_5`. Cada item abaixo vira uma seção comentada do código, na ordem dada. As variáveis declaradas no item (a) são a matéria-prima de tudo que vem depois — nenhum valor novo "cai do céu" no meio do programa.

---

## **Itens**

**a) Estado inicial.** Declare, com anotação de tipo explícita em todas:

* As três coordenadas de mundo do voxel quebrado: `mundo_x = -70`, `mundo_y = 5`, `mundo_z = -1`, no tipo do projeto para coordenadas de mundo.  
* `lado = 32`, no tipo do projeto para dimensões que servem à indexação.  
* O ID do ar (`0`) e o ID do voxel quebrado (`2`), no tipo do projeto para IDs.  
* A paleta de durezas: um array de 4 elementos do tipo de ID, com os valores `[0, 3, 5, 8]` (posição na paleta \= ID do bloco; o valor \= dureza).  
* A coordenada X da câmera: `cam_x = -64`, mesmo tipo das coordenadas de mundo.  
* Duas coordenadas para o experimento de precisão do item (f): `longe_x = 16_777_217` e `cam_longe_x = 16_777_000`, mesmo tipo das coordenadas de mundo.

Cada declaração leva um comentário curto justificando o tipo escolhido (mesmo padrão da atividade 1.2.2). Decida conscientemente quais precisam de `mut` — e lembre do warning que `mut` sem reatribuição gera.

**b) O caminho errado, documentado.** Antes de corrigir, registre a cilada: compute o chunk e o local de `mundo_x` usando os operadores **`/` e `%`** da Aula 1.2.4. Atenção: esses operadores exigem que os dois operandos tenham o mesmo tipo — aqui acontece a **primeira conversão** do programa, sobre `lado`, e ela é tecnicamente estreitante; o comentário deve dizer por que o domínio a torna segura. Imprima o par obtido e escreva um comentário explicando **qual** dos dois valores aponta para o bloco errado e **qual** nem sequer é uma posição válida — e qual seria o sintoma visual disso no jogo.

**c) Ponte 1 — o caminho certo.** Compute chunk e local dos **três eixos** com as ferramentas corretas desta aula, produzindo seis variáveis (`chunk_x/y/z`, `local_x/y/z`), todas ainda no tipo de coordenada. Imprima os três pares. Em seguida, **prove** o resultado do eixo X: derive um bool que verifica a identidade fundamental da divisão euclidiana (quociente, lado e resto reconstruindo `mundo_x`), usando comparação de igualdade, e imprima-o. O rótulo do `println!` deve dizer exatamente o que o bool afirma — precisão de rótulo é critério de correção.

**d) Ponte 2 — índice plano.** Converta os três locais para o tipo de índice e compute o índice plano com a fórmula canônica do projeto (`x + y·lado + z·lado²`). A conversão dos locais é a mesma operação perigosa que o item (e) vai expor — o comentário aqui deve dizer **qual linha do item (c) garante** que ela é segura neste ponto. Imprima o índice. Depois, derive um bool confirmando que o índice está dentro da capacidade do chunk (compare com o total de voxels, calculado a partir de `lado`) e imprima-o.

**e) O índice gigante.** Demonstre o perigo que o item (d) evitou: converta `mundo_x` **cru** (sem etapa euclidiana) para o tipo de índice e imprima o resultado com um rótulo honesto (algo como "índice se a conversão viesse antes da garantia"). Comentário: nomeie o mecanismo da Parte 1 que produz esse número e diga o que aconteceria se ele indexasse o array de voxels — em debug **e** em release.

**f) Ponte 3 — o delta antes do cast.** Usando `longe_x` e `cam_longe_x`, compute a posição de GPU **dos dois jeitos**: primeiro o errado (converta cada coordenada para o tipo de GPU e subtraia depois), depois o certo (subtraia no tipo inteiro e converta só o delta). Imprima os dois resultados e um bool comparando-os por igualdade. Um dos raros casos em que `==` entre floats é aceitável — o comentário deve justificar por quê, e dizer qual dos dois valores é o verdadeiro `217`.

**g) Consulta à paleta.** Derive `eh_solido` comparando o ID do voxel quebrado com o ID do ar (Aula 1.2.3 — solidez é derivada, nunca armazenada). Em seguida busque a dureza do voxel na paleta, convertendo o ID para o tipo de índice com **a ferramenta de garantia mais forte que existe para essa travessia** — o comentário deve dizer qual é e por que aqui ela existe, enquanto nos itens (b) e (d) não existia. Imprima `eh_solido` e a dureza.

**h) Portão final com short-circuit.** Feche o programa com a decisão composta: derive `pode_processar_quebra`, combinando com `&&` o bool de índice válido (item d) e `eh_solido` (item g), **nesta ordem** — o comentário deve explicar, nos termos da Aula 1.2.4, por que a guarda de validade vem à esquerda quando o operando direito envolve um acesso indexado (aqui ambos já são bools prontos, mas o comentário deve deixar claro qual ordem seria obrigatória se o acesso à paleta estivesse embutido na expressão). Imprima o resultado.

---

## **Regras que entram na correção**

1. Compila sem warnings; nenhum `unwrap()`/`expect()`.  
2. **Todo `as` carrega comentário** dizendo a categoria (alargamento / estreitamento / reinterpretação / int→float) e por que o domínio o torna seguro — um `as` sem justificativa é erro, mesmo com o programa funcionando.  
3. `From`/`Into` obrigatório onde a garantia existe; usá-lo onde não existe (ou `as` onde `From` existe) é erro de ferramenta.  
4. Valores impressos corretos (serão conferidos contra o gabarito calculado).  
5. Rótulos de `println!` e comentários **exatos** — chamar fato atômico de decisão final, ou rotular o valor errado de "correto", desconta (padrão já vigiado desde a 1.2.3).  
6. Lógica correta, não apenas saída correta: derivações por comparação onde se pede comparação, atribuição direta onde se pede atribuição.

## **Fixação (responder após o código aprovado, sem consultar o material)**

1. No item (f), os dois resultados divergem por exatamente `1.0`. Reconstrua a cadeia causal: o que aconteceu com `16_777_217` no momento do cast, e por que `16_777_000` saiu ileso?  
2. O item (d) depende do item (c) para ser seguro. Se um refactor futuro trocasse `rem_euclid` por `%` mantendo todo o resto igual, **qual item quebraria primeiro em runtime** e com qual comportamento observável?

Pode começar. Apresente o código quando compilar limpo; a correção segue as regras discursivas — aponto onde está errado e você tenta de novo antes de eu revelar qualquer resposta.

