# rust-estudo — VNP (Voxel Ninja Project)

Projeto de aprendizado de Rust estruturado em Fases > Blocos > Aulas.
O objetivo prático é construir um voxel engine chamado VNP.

## Stack técnica

- wgpu, winit, cpal, gilrs, directories
- Cross-platform: Linux e Windows (sem APIs específicas de plataforma)
- Caminhos via std::path::PathBuf

## Como executar um binario

    cargo run --bin aula_1_2_1

## Estrutura

    src/bin/bloco_1_2/   -> binarios das aulas do Bloco 1.2
    docs/fase-1/         -> anotacoes, pseudocodigos e entregaveis teoricos
