# Convenções de Código — rust-estudo / LVE (VNP)

> Padrão de nomenclatura de identificadores. Vale a partir da sua adoção.
> Cobrável em code review. Este arquivo é a fonte canônica; qualquer
> discrepância no código é o código que está errado, não este documento.

---

## 1. Idioma e casing

Todo identificador — variável, binding, campo, função, `const`, tipo — em **inglês**.

| Categoria de identificador      | Casing                  | Exemplo             |
|---------------------------------|-------------------------|---------------------|
| variável / binding / campo      | `snake_case`            | `voxel_size`        |
| função / método                 | `snake_case`            | `load_chunk`        |
| `const` / `static`              | `SCREAMING_SNAKE_CASE`  | `VOXEL_SIZE`        |
| tipo (struct / enum / trait)    | `PascalCase`            | `ChunkMesh`         |

---

## 2. Ordem dos segmentos: ampla → específica

O nome vai da **categoria mais ampla (à esquerda)** ao **detalhe mais específico
(à direita)**. Quanto mais à esquerda, maior o escopo; quanto mais à direita,
mais específico.

O **número de segmentos é livre** — o segmento de grupo intermediário é opcional:

- 2 segmentos: `voxel_size`, `chunk_side`, `camera_yaw`
- 3 segmentos: `enemy_ghost_hp`, `player_inventory_capacity`, `enemy_skeleton_speed`

**Por que esta ordem** (e não a ordem natural do inglês falado): identificadores
com o mesmo prefixo se agrupam no sort alfabético e no autocomplete do
rust-analyzer. Digitar `enemy_` traz `enemy_ghost_hp`, `enemy_ghost_speed`,
`enemy_skeleton_hp` juntos. O objetivo é agrupamento de máquina, não leitura
corrida — por isso os nomes leem-se um pouco "de trás para frente".

---

## 3. Campos de struct — exceção deliberada

Dentro de uma struct, **o tipo já nomeia a categoria**. O campo dropa esse prefixo.

```rust
// ERRADO — categoria repetida: player.player_inventory_capacity
struct Player {
    player_inventory_capacity: u32,
    player_hp: u32,
}

// CERTO — o tipo Player já fornece a categoria
struct Player {
    inventory_capacity: u32,
    hp: u32,
}
```

Motivo: `player.player_inventory_capacity` é redundante e anti-idiomático; o
Clippy sinaliza prefixo/sufixo repetindo o nome do tipo (lint
`clippy::struct_field_names` — confirmar o nome exato na versão fixada no
`Cargo.toml`; o princípio é estável independente do lint).

A regra da Seção 2 (prefixo de categoria) vale para **bindings livres (`let`) e
dados soltos**, não para campos de struct.

---

## 4. `const`

`SCREAMING_SNAKE_CASE`, com a mesma ordenação ampla → específica da Seção 2.

```rust
const VOXEL_SIZE: usize = 32;   // aresta do voxel / lado do chunk
const CHUNK_VOLUME: usize = VOXEL_SIZE * VOXEL_SIZE * VOXEL_SIZE;
```

---

## 5. bool — predicados com `is_` / `has_`

Variáveis e campos booleanos carregam `is_` / `has_` no segmento específico,
mantendo o prefixo de categoria à esquerda:

```rust
let voxel_is_solid: bool = voxel_id != AIR_ID;
let neighbor_is_air: bool = neighbor_id == AIR_ID;
let chunk_is_dirty: bool = true;
```

---

## 6. Categorias de topo — vocabulário controlado

O agrupamento só funciona se as categorias de topo forem um conjunto fechado.
Não use `mob_` num arquivo e `enemy_` noutro. Lista inicial:

```
player   enemy   voxel   chunk   world
camera   render  gpu     audio   input
asset    mesh
```

Categoria nova entra **por decisão consciente** (edite esta lista no mesmo commit),
nunca ad-hoc no meio de um arquivo.

---

## 7. Abreviações — allowlist

Fora desta lista, use a **palavra inteira** (`position`, não `pos`, a menos que
esteja na lista). Lista inicial de abreviações aceitas:

```
id   hp   xp   pos   vel   dir   gpu   hud   ui   aabb
```

Expandir a allowlist segue a mesma regra da Seção 6: por decisão registrada aqui.

---

## 8. Aplicação e retrofit

Vale **a partir da adoção desta convenção**. Código já corrigido e aprovado
**não é reescrito**:

- `aula_1_2_2`, `aula_1_2_3`, `aula_1_2_4` permanecem em português.
- `aula_1_2_5` em diante segue esta convenção (inglês).

---

## Referência rápida

| Situação                        | Forma                                   |
|---------------------------------|-----------------------------------------|
| binding livre                   | `enemy_ghost_hp`, `voxel_size`          |
| campo de struct                 | `hp`, `inventory_capacity` (sem prefixo)|
| constante                       | `VOXEL_SIZE`                            |
| booleano                        | `voxel_is_solid`, `chunk_is_dirty`      |
| abreviação fora da allowlist    | palavra inteira (`velocity`, não `velo`)|
