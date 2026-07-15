# rust-estudo

Repositório de estudos de Rust estruturado em **Fases → Blocos → Aulas**.
O objetivo é acumular o conhecimento necessário para, no futuro, construir
a **LVE (Lena Voxel Engine)** — também referida como VNP (Voxel Nightmare Project).

---

## Objetivo

Construir fundamentos sólidos de programação usando Rust como veículo.
Os exemplos e atividades práticas são contextualizados na LVE desde o início,
de modo que o aprendizado já segue as decisões de design da engine.

- **Fase 1** — Fundamentos de programação com Rust (em andamento)
- **Fase 2** — Rust intermediário focado no projeto (pendente)

---

## Stack técnica (LVE)

| Função          | Crate          |
|-----------------|----------------|
| Renderização    | `wgpu`         |
| Janela / input  | `winit`        |
| Áudio           | `cpal`         |
| Gamepad         | `gilrs`        |
| Diretórios      | `directories`  |

**Restrições cross-platform:** sem APIs específicas de plataforma (Win32,
X11, Wayland, ALSA direto). `std::path::PathBuf` para todos os caminhos;
tratamento explícito de quebras de linha em I/O de texto; nomes de arquivo
com caixa consistente. Alvos: Linux e Windows desde o início.

---

## Decisões de tipo já fixadas

Estabelecidas ao longo do Bloco 1.2 e válidas para toda a engine:

| Domínio                        | Tipo     | Razão                                                        |
|--------------------------------|----------|--------------------------------------------------------------|
| Coordenada de mundo / chunk    | `i32`    | Assinada, exata, faixa suficiente                            |
| Índice plano do array de voxel | `usize`  | Exigido pela trait `Index`                                   |
| ID de voxel                    | `u8`     | Paleta de 256 tipos; 4× menos memória que `u32`              |
| Vértice / posição na GPU       | `f32`    | Tipo nativo da GPU                                           |

- Solidez é **derivada** do ID (`id != 0`), nunca armazenada em paralelo.
- `usize` é **proibido** em struct `#[repr(C)]` voltada à GPU (largura dependente de plataforma).
- Mundo → chunk/local usa `div_euclid`/`rem_euclid`; `/` e `%` truncam em direção a zero e quebram no octante negativo.
- Recentragem de câmera: subtrair em `i32` primeiro, converter só o delta para `f32`.

---

## Como executar um binário

```sh
cargo run --bin aula_1_2_1
cargo run --bin aula_1_2_5
```

Para o perfil de release (sem overflow checks, wrapping behavior):

```sh
cargo run --release --bin aula_1_2_2
```

---

## Estrutura de arquivos

```
rust-estudo/
├── Cargo.toml                        # autobins = false; [[bin]] explícitos
├── Cargo.lock
├── README.md
├── QuestoesObjetivas/
│   └── Bloco1_2/                     # avaliações e atividades discursivas
│       ├── aula_1_2_5_avaliacao.md
│       └── aula_1_2_5_discursiva.md
└── src/
    └── bin/
        └── bloco_1_2/                # binários das aulas do Bloco 1.2
            ├── aula_1_2_1.rs
            ├── aula_1_2_2.rs
            ├── aula_1_2_3.rs
            ├── aula_1_2_4.rs
            └── aula_1_2_5.rs
```

> A partir do Bloco 1.2, cada aula é um binário `.rs` com entrada `[[bin]]` explícita no `Cargo.toml`.
> As avaliações objetivas e as atividades discursivas ficam em `QuestoesObjetivas/Bloco1_x/`.

---

## Progresso

### Fase 1 — Fundamentos de programação com Rust

#### Bloco 1.1 — Fundamentos de computação e pensamento programático ✅

| Aula   | Título                                            | Status |
|--------|---------------------------------------------------|:------:|
| 1.1.1  | O que é um Programa                               | ✅ |
| 1.1.2  | Compilação versus Interpretação                   | ✅ |
| 1.1.3  | Código de Máquina e Assembly                      | ✅ |
| 1.1.4  | Sistemas Numéricos                                | ✅ |
| 1.1.5  | Codificação de Texto: ASCII e UTF-8               | ✅ |
| 1.1.6  | Memória: Stack e Heap (conceitual)                | ✅ |
| 1.1.7  | Pseudocódigo                                      | ✅ |
| 1.1.8  | Decomposição de Problemas                         | ✅ |
| 1.1.9  | Intuição de Complexidade (Big-O)                  | ✅ |
| 1.1.10 | Lógica Booleana e Estruturas de Decisão           | ✅ |
| 1.1.11 | Fechamento do Bloco + Entregável (pseudocódigos)  | ✅ |

**Entregável do bloco:** cinco pseudocódigos (FizzBuzz, reverter string, contar
vogais, encontrar o maior, palíndromo), a serem traduzidos para Rust na Aula 1.2.11.

#### Bloco 1.2 — Sintaxe, tipos e controle de fluxo (em andamento)

| Aula   | Título                                                       | Binário       | Status |
|--------|--------------------------------------------------------------|---------------|:------:|
| 1.2.1  | Variáveis, Imutabilidade e o Primeiro Binário                | `aula_1_2_1`  | ✅ |
| 1.2.2  | Tipos Numéricos: Inteiros e Ponto Flutuante                  | `aula_1_2_2`  | ✅ |
| 1.2.3  | `bool` e `char`                                              | `aula_1_2_3`  | ✅ |
| 1.2.4  | Operadores                                                   | `aula_1_2_4`  | ✅ |
| 1.2.5  | Conversão de Tipos: `as`, `From`/`Into`, `TryFrom`/`TryInto`, `div_euclid`/`rem_euclid` | `aula_1_2_5` | ✅ |
| 1.2.6  | `const` e Constantes vs Variáveis Imutáveis                  | —             | ⬜ |
| 1.2.7  | Decisão: `if` / `else if` / `else`                           | —             | ⬜ |
| 1.2.8  | `match` Básico                                               | —             | ⬜ |
| 1.2.9  | Loops e Controle de Iteração                                 | —             | ⬜ |
| 1.2.10 | Blocos como Expressão e Escopo                               | —             | ⬜ |
| 1.2.11 | Fechamento do Bloco + Entregável (cinco pseudocódigos em Rust) | —           | ⬜ |

**Blocos seguintes da Fase 1 (pendentes):** 1.3 Funções, escopo e recursão · 1.4
Ownership e borrowing · 1.5 Structs, enums e pattern matching · 1.6 Tratamento de
erros · 1.7 Coleções padrão · 1.8 Algoritmos e estruturas de dados · 1.9
Organização de código, ferramentas e debugging.

---

## Cargo.toml — notas relevantes

- `autobins = false`: nenhum binário é detectado automaticamente.
- Todo novo binário exige entrada `[[bin]]` explícita com `name` e `path`.
- `[profile.dev]` tem `overflow-checks = true` (panic em overflow).
- `[profile.release]` tem `overflow-checks = false` (wrapping silencioso).
- A diferença entre os dois perfis é material didático desde a Aula 1.2.2 — não altere sem motivo.

---

## Pendências conhecidas do repositório

- Não há CI. A estratégia prevista é GitHub Actions compilando para `linux-gnu`
  e `windows-gnu` assim que existir uma base funcional da engine.
- Os documentos de currículo (`fase-1-detalhada.md`, `fase-2-detalhada.md`) e as
  atividades do Bloco 1.1 ainda não estão versionados neste repositório.