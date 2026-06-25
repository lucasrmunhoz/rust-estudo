# rust-estudo

Repositório de estudos de Rust estruturado em **Fases → Blocos → Aulas**.
O objetivo é acumular o conhecimento necessário para, no futuro, construir
a **LVE (Lena Voxel Engine)**.

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
X11, Wayland, ALSA direto). `std::path::PathBuf` para todos os caminhos.
Alvos: Linux e Windows desde o início.

> **VNP** que aparece nos arquivos e comentários de código gerados durante o
> curso é uma sigla legada. O nome oficial da engine é **LVE — Lena Voxel Engine**.

---

## Como executar um binário

```sh
cargo run --bin aula_1_2_1
cargo run --bin aula_1_2_2
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
└── src/
    └── bin/
        └── bloco_1_2/                # binários das aulas do Bloco 1.2
            ├── aula_1_2_1.rs
            ├── aula_1_2_2.rs
            └── aula_1_2_3.rs
```

> A partir do Bloco 1.2, cada aula é um binário `.rs` com entrada `[[bin]]` explícita no `Cargo.toml`.

---

## Progresso

### Fase 1 — Fundamentos de programação com Rust

#### Bloco 1.1 — Fundamentos de computação e pensamento programático ✅

| Aula   | Título                                            
|--------|---------------------------------------------------
| 1.1.1  | O que é um Programa                              
| 1.1.2  | Compilação versus Interpretação                   
| 1.1.3  | Código de Máquina e Assembly                   
| 1.1.4  | Sistemas Numéricos                              
| 1.1.5  | Codificação de Texto: ASCII e UTF-8             
| 1.1.6  | Memória: Stack e Heap (conceitual)              
| 1.1.7  | Pseudocódigo                                   
| 1.1.8  | Decomposição de Problemas                        
| 1.1.9  | Intuição de Complexidade (Big-O)                  
| 1.1.10 | Lógica Booleana e Estruturas de Decisão           
| 1.1.11 | Fechamento do Bloco + Entregável (pseudocódigos)  

#### Bloco 1.2 — Sintaxe, tipos e controle de fluxo (em andamento)

| Aula   | Título                                           
|--------|---------------------------------------------------
| 1.2.1  | Variáveis, Imutabilidade e o Primeiro Binário     
| 1.2.2  | Tipos Numéricos: Inteiros e Ponto Flutuante       
| 1.2.3  | `bool` e `char`                                   
| 1.2.4  | Operadores                                       
| 1.2.5  | Conversão de Tipos: `as`, `From`, `Into`         
| 1.2.6  | `const` e Constantes vs Variáveis Imutáveis       
| 1.2.7  | Decisão: `if` / `else if` / `else`                
| 1.2.8  | `match` Básico                                   
| 1.2.9  | Loops e Controle de Iteração                     
| 1.2.10 | Blocos como Expressão e Escopo                    
| 1.2.11 | Fechamento do Bloco + Entregável 

---

## Cargo.toml — notas relevantes

- `autobins = false`: nenhum binário é detectado automaticamente.
- Todo novo binário exige entrada `[[bin]]` explícita com `name` e `path`.
- `[profile.dev]` tem `overflow-checks = true` (panic em overflow).
- `[profile.release]` tem `overflow-checks = false` (wrapping silencioso).