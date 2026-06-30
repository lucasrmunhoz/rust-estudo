fn main() {
    let id_atual:u8 = 1;
    let id_cima:u8 = 0;
    let id_baixo:u8 = 1;
    let id_direita:u8 = 0;
    let id_esquerda:u8 = 1;
    let id_ar:u8 = 0;

    // A1 Objetivo: produzir um bool a partir de uma comparação, distinguindo "sólido" de "ar".
    // Compute atual_eh_solido: bool — verdadeiro quando id_atual representa um bloco sólido (compare com o ID do ar). Imprima.
    let atual_eh_solido:bool = id_atual != id_ar; // bool criado comparado ids diferentes resultado true
        println!("atual_eh_solido: {}", atual_eh_solido);


    // A2 Objetivo: isolar, por vizinho, o fato booleano que alimenta a visibilidade da face.
    // Compute quatro bools — vizinho_cima_eh_ar, vizinho_baixo_eh_ar, vizinho_esquerda_eh_ar, vizinho_direita_eh_ar
    // cada um verdadeiro quando aquele vizinho é ar. Imprima os quatro.
    // (A decisão final "emitir a face de cima" é atual_eh_solido e vizinho_cima_eh_ar combinados — a combinação com && é a Aula 1.2.4. Aqui você entrega só as peças.)

    let vizinho_cima_eh_ar:bool = id_cima == id_ar; // variavel bool criada comparando voxel vizinho com o ar
        println!("Vizinho de cima é: {}", vizinho_cima_eh_ar);
    let vizinho_baixo_eh_ar:bool = id_baixo == id_ar;
        println!("Vizinho de baixo é: {}", vizinho_baixo_eh_ar);
    let vizinho_direita_eh_ar:bool = id_direita == id_ar;
        println!("Vizinho de direita é: {}", vizinho_direita_eh_ar);
    let vizinho_esquerda_eh_ar:bool = id_esquerda == id_ar;
        println!("Vizinho de esquerda é: {}", vizinho_esquerda_eh_ar);
    // reflexao: a face precisa de dois fatos, saber se o voxel atual é solido (no inicio para evitar contas complexas de cara) e se o vizinho é ar para identifcar a face visivel, a combinacao dos dois fatos com && vai gerar o resultado final de visibilidade da face

    // A3 Objetivo: usar == para uma decisão de meshing distinta — fusão de tipos (greedy meshing).
    // Compute mesmo_tipo_que_baixo: bool — verdadeiro quando o voxel atual e o vizinho de baixo são do mesmo tipo. Imprima.
    // (Greedy meshing só pode fundir faces de blocos do mesmo tipo; este bool é o pré-requisito dessa fusão.)
    let mesmo_tipo_cima:bool = id_atual == id_cima; // bool criado comparando voxel atual com os viznhos
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
    let mut chunk_sujo:bool = false; // caso explicito de false para criar variavel, mut usado para reatribuir valor
        println!("chunk_sujo inicial: {}", chunk_sujo);
    chunk_sujo = true; // reatribuicao de valor para true
        println!("chunk_sujo final: {}", chunk_sujo);


    // B1 Objetivo: declarar char com aspas simples e fixar a distinção char vs &str.
    // Declare, todos : char: digito_hud = '7' (um dígito de coordenada no HUD), inicial_bloco = 'P' (inicial de "pedra"), e os dois caracteres de quebra de linha nova_linha = '\n' e retorno = '\r'.
    // Imprima os quatro. Num comentário, explique por que '7' é um char e "7" seria um &str.
    let digito_hud:char = '7'; // tipo char para caracteres, usado para representar um digito
        println!("digito_hud: {}", digito_hud);
    let inicial_bloco: char = 'P';
        println!("inicial_bloco: {}", inicial_bloco); 
    let nova_linha: char = '\n';
        println!("nova_linha: {}", nova_linha);
    let retorno: char = '\r';
        println!("retorno: {}", retorno);
    // aspas simples para char, aspas duplas para string pela documentacao do rust, char é um tipo de dado que representa um unico caractere, string é uma sequencia de caracteres

    //B2 Objetivo: obter o code point via as u32 — a conversão sempre segura.
    // Compute cp_inicial: u32 (code point de inicial_bloco) e cp_nova_linha: u32 (code point de nova_linha), usando as u32. Imprima ambos.
    let cp_inicial: u32 = inicial_bloco as u32; // conversao de char para u32 para obter o codigo unicode do caractere
        println!("cp_inicial: {}", cp_inicial);
    let cp_nova_linha: u32 = nova_linha as u32;
        println!("cp_nova_linha: {}", cp_nova_linha);

    // B3 Objetivo: classificar caracteres no parser de config, produzindo bool — a ponte entre as duas metades da aula.
    // Compute digito_eh_numerico: bool (se digito_hud é um dígito ASCII) e inicial_eh_letra: bool (se inicial_bloco é alfabético), usando os métodos de char apropriados. Imprima.
    // (Repare: um parser decidindo "este caractere é um dígito?" é uma decisão bool, igual ao meshing — só que a fonte do bool é um char.)
    let digito_eh_numerico: bool = digito_hud.is_ascii_digit(); // metodo is_ascii_digit para verificar se o caractere é um digito
        println!("digito_eh_numerico: {}", digito_eh_numerico);
    let inicial_eh_letra: bool = inicial_bloco.is_alphabetic(); // metodo is_alphabetic para verificar se o caractere é uma letra
        println!("inicial_eh_letra: {}", inicial_eh_letra);

    // B4 Objetivo: comprovar que transformar um char retorna um valor NOVO e não muta o original.
    // Declare inicial_minuscula: char = 'p'. Compute inicial_maiuscula: char aplicando a versão ASCII de maiusculização sobre inicial_minuscula
    // Imprima os dois: a maiúscula obtida e a inicial_minuscula original. O ponto da tarefa é você observar na saída que o original continua 'p'.
    let inicial_minuscula:char = 'p'; // char minusculo para testar o metodo to_ascii_uppercase
        println!("inicial_minuscula: {}", inicial_minuscula);
    let inicial_maiuscula:char = inicial_minuscula.to_ascii_uppercase(); // metodo to_ascii_uppercase para converter char minusculo para maiusculo
        println!("inicial_maiuscula: {}", inicial_maiuscula);
        println!("A inicial minuscula {} manteve seu dado original mesmo apos manipulacao", inicial_minuscula);
    
}