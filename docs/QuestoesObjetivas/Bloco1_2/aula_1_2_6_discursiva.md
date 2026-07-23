# **Aula 1.2.6 — Atividade Prática (questão discursiva)**

## **Cenário**

O VNP precisa alocar o armazenamento de um chunk pela primeira vez. Até agora, todos os seus binários trataram voxels como variáveis soltas — um `id_atual`, um `id_vizinho`, um array de cinco elementos para testar curto-circuito. Agora o chunk existe de verdade: 32.768 voxels contíguos em memória, dimensionados pela constante que sustenta toda a arquitetura.

Esse array **não pode** ser declarado com `let`. Não por convenção, não por elegância — por impossibilidade sintática. É a primeira vez no curso em que uma ferramenta que você já domina simplesmente não cabe na posição, e o programa desta aula existe para você provar isso ao compilador e ouvir a recusa dele com as próprias mãos.

## **Objetivo**

Aplicar, num único programa contínuo: consumo da casa das constantes via `use`; declaração de `const` local com anotação de tipo e derivação em compile-time a partir de outra constante; dimensionamento de array por constante; curadoria consciente entre valor canônico e literal ad-hoc; escopo de `const` em módulo e em bloco; e o registro documentado de quatro recusas do compilador que só existem no território de `const`.

O único andaime de aulas anteriores admitido é: anotação explícita de tipo (1.2.2), `let mut` para o array (1.2.1) e uma única aplicação da fórmula do índice plano (1.2.4). Nada de conversão euclidiana, paleta, curto-circuito ou coordenadas negativas — se você sentir vontade de trazer qualquer um desses, está saindo do escopo.

## **O que entregar**

O binário `src/bin/bloco_1_2/aula_1_2_6.rs` (substituindo integralmente o stub atual), com entrada `[[bin]]` registrada no `Cargo.toml` — que ainda **não existe**, e é sua a responsabilidade de acrescentá-la ao final da lista, mantendo o padrão dos anteriores. O programa deve compilar **sem warnings** via `cargo run --bin aula_1_2_6`.

Cada item abaixo vira uma seção comentada do código, na ordem dada.

---

## **Itens**

### **a) Curadoria — de onde vem cada valor**

Antes de escrever qualquer linha executável, decida a procedência de cada valor que o programa vai usar. São três procedências possíveis:

* **Casa das constantes** — importado com `use rust_estudo::consts::{...}`;
* **`const` local** — declarado no topo deste arquivo, fora do `main`;
* **`let` comum** — dentro do `main`.

Os valores que o programa precisa:

1. A aresta do chunk em voxels;
2. A área de uma camada do chunk;
3. O total de voxels do chunk;
4. A aresta do chunk no tipo `i32`;
5. O ID do ar;
6. O número de faces de um cubo (`6`);
7. O total de faces que o meshing ingênuo emitiria num chunk cheio;
8. As três coordenadas locais do voxel que o programa vai escrever;
9. O ID do voxel que o programa vai escrever.

Escreva a lista de importação e as declarações locais que essa decisão produz, e em comentário justifique **dois** casos: um que você classificou como canônico e um que classificou como ad-hoc, nomeando o critério que os separa.

Atenção: importar da casa algo que o programa não usa tem consequência mecânica — se você não sabe qual, descubra compilando.

---

### **b) Derivação em compile-time**

Declare, como `const` local de escopo de módulo, o total de faces do meshing ingênuo. Ele deve ser **derivado por expressão** a partir das constantes já disponíveis — nenhum dígito além do `6` pode aparecer nessa linha, e o número ≈196.000 nunca é digitado.

Escolha o nome seguindo sua convenção de identificadores. Em comentário, registre a cadeia de dependência (qual constante deriva de qual, até a decisão de projeto original) e responda: se `CHUNK_SIDE` mudasse para `16`, quantas linhas do seu programa precisariam ser editadas?

Imprima o valor.

---

### **c) O array que exige `const`**

Declare o armazenamento do chunk: um array de IDs de voxel, dimensionado pelo total de voxels e preenchido inteiramente com o ID do ar.

Compute o índice plano do voxel do item (a.8) usando a fórmula canônica do projeto, aproveitando as constantes disponíveis — a área não deve ser recalculada inline se já existe nomeada. Escreva o ID do item (a.9) nessa posição, releia o valor dali e imprima os dois: o índice e o ID recuperado.

Imprima também o comprimento do array e um bool comparando-o com a constante que o dimensionou. Em comentário, explique por que esse bool é obrigatoriamente `true` — e por que ele mesmo assim vale ser impresso.

---

### **d) Laboratório de recusas do compilador**

Esta seção não produz saída. Ela produz conhecimento registrado.

Para cada uma das quatro provocações abaixo: escreva a linha, rode `cargo build`, **leia a mensagem inteira** do compilador, registre num comentário o código do erro (se houver um `E0xxx`) e a frase essencial da mensagem, e então comente a linha para que o arquivo volte a compilar.

* **D1.** Uma `const` sem anotação de tipo.
* **D2.** Uma `const` cujo valor vem de um `let` declarado logo acima.
* **D3.** Duas constantes `u16`: uma valendo `512`, outra derivada como o quadrado da primeira.
* **D4.** Um array dimensionado por um `let` imutável, mesmo que esse `let` valha exatamente o total de voxels.

Ao final da seção, um comentário comparando **D2 e D4**: os códigos de erro são o mesmo ou diferentes? O que isso revela sobre o mecanismo comum às duas recusas?

Um comentário adicional sobre **D3**: qual perfil de compilação seria necessário para transformar essa recusa num wrapping silencioso? Responda com precisão — a resposta não é o nome de um perfil.

**Provocação extra, de categoria diferente.** Declare uma `const` em minúsculas, rode `cargo build`, e registre em comentário o que o compilador devolveu. Compare explicitamente com as quatro anteriores: a categoria da resposta é a mesma? O binário chegou a ser gerado? Depois corrija a grafia — seu critério de entrega não admite o que apareceu ali.

---

### **e) O contraste na mesma cena**

Declare, dentro do `main`, um `let` imutável com o mesmo valor da aresta do chunk. Imprima os dois lado a lado, provando que carregam o mesmo número.

Em comentário, liste **três** coisas que a constante faz e que esse `let` não pode fazer — uma delas já demonstrada em (d), as outras duas você identifica. Depois responda: se os dois valem `32` e nunca mudam durante a execução, em que sentido exato eles são diferentes? A resposta que menciona mutabilidade está errada.

---

### **f) Escopo local**

Dentro do `main`, abra um bloco `{ }` e declare ali dentro uma `const` — algo plausível como limite pontual, por exemplo o teto de níveis de luz de um voxel. Use-a dentro do bloco e imprima.

Em comentário, fora do bloco, registre o que aconteceria se você tentasse usá-la dali (não escreva o código; apenas nomeie a categoria do erro). E responda: essa constante é avaliada em compile-time como as do topo do arquivo, ou o escopo local muda alguma coisa a esse respeito?

---

## **Regras que entram na correção**

1. Compila **sem warnings**; nenhum `unwrap()`/`expect()`.
2. **Nenhum número mágico:** todo valor derivável é derivado por expressão. Um `1024`, `32768` ou `196608` digitado à mão é erro, mesmo com o programa funcionando.
3. Toda `const` local declarada nesta aula segue a convenção de nomenclatura em inglês (`SCREAMING_SNAKE_CASE`, categoria mais ampla à esquerda). Nome fora da convenção desconta.
4. As quatro recusas de (d) precisam do **registro real** do compilador — texto genérico do tipo "dá erro porque não pode" não conta como entrega. Se um erro não trouxer código `E0xxx`, registre isso explicitamente em vez de inventar um.
5. Rótulos de `println!` e comentários **exatos**: chamar de "constante" um `let`, ou de "erro" um warning, desconta (padrão vigiado desde a 1.2.3).
6. Comentários de justificativa presentes onde os itens pedem — a decisão de curadoria em (a) e o contraste em (e) valem tanto quanto o código.
7. Lógica correta, não apenas saída correta.

---

## **Fixação (responder após o código aprovado, sem consultar o material)**

1. O VNP terá dois valores fixos: a **versão do formato de save** (um número que só muda quando os desenvolvedores alteram a serialização e recompilam) e a **semente do worldgen** (lida do arquivo de save ao carregar a partida, imutável dali em diante). Classifique cada um como `const` ou `let` imutável, e descreva o que quebraria concretamente se você trocasse as duas classificações entre si.

2. No item (d.4) o compilador recusou um array dimensionado por um `let` que valia exatamente `32768` — um valor que ele consegue enxergar na linha acima. Explique por que a recusa **não** é uma limitação do compilador, mas uma consequência de onde o comprimento do array vive na linguagem. Sua resposta precisa mencionar o que é o `N` em `[u8; N]`.

---

Pode começar. Apresente o código quando compilar limpo; a correção segue as regras discursivas — aponto onde está errado e você tenta de novo antes de eu revelar qualquer resposta.
