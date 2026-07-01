fn main() {
    // ==========================================================
    // SEÇÃO 1 — Índice plano (ida e volta)
    // ==========================================================
    // Declare lado: usize = 32 e uma coordenada local x, y, z (usize, 0..32).
    // Calcule i = x + y*lado + z*lado*lado (sem parênteses supérfluos).
    // Recupere x, y, z de volta a partir de i usando apenas / e %.
    // Compare cada recuperado com o original usando == e imprima os três bools.
    // Os três devem dar true.

    // TODO: lado, x, y, z
        let lado:usize = 32;
        let x:usize = 3; //valores arbritarios usados em x y z
        let y:usize = 12;
        let z:usize = 7;

    // TODO: i
        let i: usize = x + y*lado + z*lado*lado;
                println!("O valor do indice eh {}", i);

    // TODO: recuperar z, y, x (nessa ordem — pense em qual eixo "sai" primeiro)
        let recuperado_z:usize = i / (lado*lado);
        let resto_sem_z:usize = i % (lado*lado);   
        let recuperado_y:usize = resto_sem_z / lado;
        let recuperado_x:usize = i % lado;     

    // TODO: comparações == e println! de cada uma
        println!("O valor recuperado de z eh {}, e a comparacao com o valor original z eh {}", recuperado_z, recuperado_z==z);
        println!("O valor recuperado de y eh {}, e a comparacao com o valor original y eh {}", recuperado_y, recuperado_y==y);
        println!("O valor recuperado de x eh {}, e a comparacao com o valor original x eh {}", recuperado_x, recuperado_x==x);



    // ==========================================================
    // SEÇÃO 2 — Coordenada de mundo negativa (só observação)
    // ==========================================================
    // Declare mundo: i32 = -1 e lado_i: i32 = 32.
    // Imprima mundo / lado_i e mundo % lado_i.
    // Imprima também 31 / lado_i e 31 % lado_i para contraste.
    // NÃO corrija nada aqui — só observe e comente o resultado.

    // TODO: mundo, lado_i
    // TODO: println! de mundo / lado_i e mundo % lado_i
    // TODO: println! de 31 / lado_i e 31 % lado_i
    // Comentário: o que a intuição de grid esperava para -1, e o que Rust entregou?


    // ==========================================================
    // SEÇÃO 3 — Visibilidade de face
    // ==========================================================
    // Declare atual_id: u8 e vizinho_id: u8.
    // Derive atual_eh_solido e vizinho_eh_ar a partir dos ids (nunca armazene solidez à parte).
    // Combine em desenhar_face. Imprima os três bools.
    // Teste com pelo menos dois pares de ids que deem resultados diferentes.

    // TODO: primeiro par de ids + derivação + combinação + prints
    // TODO: segundo par de ids + derivação + combinação + prints


    // ==========================================================
    // SEÇÃO 4 — Curto-circuito na borda
    // ==========================================================
    // let voxels = [1u8, 0, 2, 0, 3];
    // Escreva dentro && voxels[idx_vizinho] != 0, onde dentro é a checagem de limite.
    // Avalie com um idx_vizinho válido e com um idx_vizinho fora do intervalo.
    // Não pode dar panic em nenhum dos dois casos. Imprima o bool resultante de cada um.

    // TODO: voxels
    // TODO: caso com idx_vizinho válido
    // TODO: caso com idx_vizinho fora do intervalo


    // ==========================================================
    // SEÇÃO 5 — Transparência (De Morgan)
    // ==========================================================
    // Declare vizinho_solido: bool e vizinho_opaco: bool.
    // desenhar_a = !(vizinho_solido && vizinho_opaco)
    // desenhar_b = !vizinho_solido || !vizinho_opaco
    // Imprima ambos e imprima desenhar_a == desenhar_b.

    // TODO: vizinho_solido, vizinho_opaco
    // TODO: desenhar_a, desenhar_b
    // TODO: println! dos dois e da comparação entre eles


    // ==========================================================
    // SEÇÃO 6 — Acumuladores do meshing
    // ==========================================================
    // vertices e faces_expostas como acumuladores (mut), simulando 3 faces emitidas:
    //   += 4 em vertices por face, += 1 em faces_expostas, três vezes.
    // nivel_luz: u8 = 200, aplique uma atenuação com -=.
    // Imprima os três valores finais.

    // TODO: vertices, faces_expostas (mut, inicializados)
    // TODO: três incrementos simulando três faces
    // TODO: nivel_luz e atenuação com -=
    // TODO: prints finais
}

// ==============================================================
// FIXAÇÃO — responda em comentário aqui embaixo antes de reportar
// ==============================================================
//
// 1. Na Seção 4, por que inverter a ordem para
//    `voxels[idx_vizinho] != 0 && dentro` reintroduziria o risco de panic?
//    Descreva o que Rust avalia primeiro em cada versão.
//
// Resposta:
//
//
// 2. Na Seção 2, por que -1 % 32 deu -1 em vez de 31? Diga qual valor de
//    coordenada local isso representaria e por que ele é impossível num
//    chunk — e nomeie a ferramenta da 1.2.5 que corrige o mapa.
//
// Resposta:
//