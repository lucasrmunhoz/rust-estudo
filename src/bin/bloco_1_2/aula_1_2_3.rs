fn main() {
    let id_atual:u8 = 1;
    let id_cima:u8 = 0;
    let id_baixo:u8 = 1;
    let id_direita:u8 = 0;
    let id_esquerda:u8 = 1;

    // A1 Objetivo: produzir um bool a partir de uma comparação, distinguindo "sólido" de "ar".
    // Compute atual_eh_solido: bool — verdadeiro quando id_atual representa um bloco sólido (compare com o ID do ar). Imprima.
    let atual_eh_solido:bool = id_atual != id_cima;
        println!("atual_eh_solido: {}", atual_eh_solido);


    // A2 Objetivo: isolar, por vizinho, o fato booleano que alimenta a visibilidade da face.
    // Compute quatro bools — vizinho_cima_eh_ar, vizinho_baixo_eh_ar, vizinho_esquerda_eh_ar, vizinho_direita_eh_ar
    // cada um verdadeiro quando aquele vizinho é ar. Imprima os quatro.
    // (A decisão final "emitir a face de cima" é atual_eh_solido e vizinho_cima_eh_ar combinados — a combinação com && é a Aula 1.2.4. Aqui você entrega só as peças.)

    let vizinho_cima_eh_ar:bool = id_atual != id_cima;
        println!("A decisão final emitir a face de cima é: {}", vizinho_cima_eh_ar);
    let vizinho_baixo_eh_ar:bool = id_atual != id_baixo;
        println!("A decisão final emitir a face de baixo é: {}", vizinho_baixo_eh_ar);
    let vizinho_direita_eh_ar:bool = id_atual != id_direita;
        println!("A decisão final emitir a face de direita é: {}", vizinho_direita_eh_ar);
    let vizinho_esquerda_eh_ar:bool = id_atual != id_esquerda;
        println!("A decisão final emitir a face de esquerda é: {}", vizinho_esquerda_eh_ar);

    // A3 Objetivo: usar == para uma decisão de meshing distinta — fusão de tipos (greedy meshing).
    // Compute mesmo_tipo_que_baixo: bool — verdadeiro quando o voxel atual e o vizinho de baixo são do mesmo tipo. Imprima.
    // (Greedy meshing só pode fundir faces de blocos do mesmo tipo; este bool é o pré-requisito dessa fusão.)
    let mesmo_tipo_cima:bool = id_atual == id_cima;
        println!("greedy meshing cima: {}", mesmo_tipo_cima);
    let mesmo_tipo_baixo:bool = id_atual == id_baixo;
        println!("greedy meshing baixo: {}", mesmo_tipo_baixo);
    let mesmo_tipo_direita:bool = id_atual == id_direita;
        println!("greedy meshing direita: {}", mesmo_tipo_direita);
    let mesmo_tipo_esquerda:bool = id_atual == id_esquerda;
       println!("greedy meshing esquerda: {}", mesmo_tipo_esquerda);

    // A4 Objetivo: exercitar reatribuição de bool como estado mutável de chunk.
    // Declare chunk_sujo: bool iniciando em false (um chunk recém-meshado está limpo). Imprima.
    // Em seguida, simule o jogador quebrando um voxel reatribuindo chunk_sujo para true (sinaliza que o chunk precisa ser remeshado).
    // Imprima de novo. (Este é um mut legítimo — de fato reatribuído; não deve gerar unused_mut.)
    let mut chunk_sujo:bool = false;
        println!("chunk_sujo inicial: {}", chunk_sujo);
    chunk_sujo = id_atual != id_cima;
        println!("chunk_sujo final: {}", chunk_sujo);


    // B1
    let digito_hud:char = '7';
}