fn main() {
//item A
    // i32: Permite valores negativos e tem precisão suficiente para coordenadas espaciais padrão
    let world_x: i32 = -70;
    let world_y: i32 = 5;
    let world_z: i32 = -1;
        println!("Coordenadas de mundo X={} Y={} Z={}", world_x, world_y, world_z);
    
    // usize: Tipo ideal para representar tamanhos, dimensões de matrizes e indexação na arquitetura alvo
    let side: usize = 32;  

    // u8: Economiza memória (1 byte), pois identificadores de blocos básicos são pequenos e não negativos
    let id_air: u8 = 0;   
    // u8: Mantém a consistência com outros identificadores de blocos, otimizando o uso de memória
    let id_broken: u8 = 2;
    // [u8; 4]: Array de u8 para economizar memória, pois os valores de resistência são pequenos; 
    // a conversão para usize será feita apenas no momento da indexação (item g)
    let tough: [u8; 4] = [0, 3, 5, 8];

    // i32: Permite valores negativos e garante compatibilidade com as coordenadas de mundo
    let cam_x: i32 = -64;
    // i32: Suporta valores grandes (até ~2 bilhões), valor escolhido para representar o primeiro inteiro que f32 deixa de representar exatamente
    let far_x: i32 = 16_777_217;
    // i32: Mantém a consistência de tipo com longe_x para permitir comparações e cálculos diretos, escolhido deliberadamente para testar a fronteira de 2²⁴ no cast para f32 
    let cam_far_x: i32 = 16_777_000;
        println!("side {}, id air {}, id broken {}, tough {:?}, cam x {}, longe x {}, cam longe x {}"
        , side, id_air, id_broken, tough, cam_x, far_x, cam_far_x);
//item B
    // i32 para representar chunks em posicoes inteiras positivas e negativas, a conta correta usa div euclid para o calculo correto do chunk
    // as justificado para tranformar side que eh usize pequeno para i32 para a divisao com world_x i32 
    // estreitamento de side de 8bytes para 4bytes - valor 32 cabe com folga nos dois, usize $0$ a $2^{64} - 1$ (em 64-bit) e i32 $-2.147.483.648$ a $2.147.483.647$ 
   // let chunk_x: i32 = world_x / side as i32; // esperado -3 com a conta correta div euclid
    // u8 seria o correto pois coordenadas locais sao sempre 0 a 31 quando usando rem euclid corretamente
    // i8 usado pois o teste para mostrar o calculo errado gera uma coord local negativa impossivel
    // as usado na expressao para trandormar world_x que eh pequeno e i32 para i8 e side que eh usize pequeno para i8, os dois no mesmo tipo para que % funcione corretamente 
    // estreitamento de world x de 4bytes para 1byte valor -70 ainda cabe, estreitamento de side usize 8bytes para i8 1byte valor 32 cabe ainda, usize = $0$ a $2^{64} - 1$ , i32 = $-2.147.483.648$ a $2.147.483.647$ e i8 = $-128$ a $127$ 
   // let local_x: i8  = world_x as i8 % side as i8; // esperado 26 com rem.euclid
   //     println!("conta errada do chunk x = {} e local x = {}", chunk_x, local_x);
    // chunk_x aponta para o chunk errado pois / em coordenadas negativas tem o arrendondamento para o valor inteiro proximo de 0 (-2,18 para -2)
    // local_x aponta para uma coordenada imposivel pois coords local vao de 0 a 31 e % eh o resto de / ((-2 . 32) -6 = -70)
    // o sintoma no jogo seria a busca do chunk errado afetando os voxel do chunk -2 e a busca da coord local -6 causa panic por busca fora do index
    // antes do jogo travar o jogador veria bugs visuais nos voxels do chunk -2

//item C
    // i32 usado pois chunks podem ser grande e negativos
    // as usado como estreitamento, side de valor 32 tipo usize 8bytes 2^64 cabe em i32 4bytes 2^31 -1
    let chunk_x: i32 = world_x.div_euclid(side as i32);
    let chunk_y: i32 = world_y.div_euclid(side as i32);
    let chunk_z: i32 = world_z.div_euclid(side as i32);
        println!("Chunk x = {}, y = {}, z = {}", chunk_x, chunk_y, chunk_z);

    // i32 usado pois rem euclide exige i32
    // as usado como estreitamento, side de valor 32 tipo usize 8bytes 2^64 cabe em i32 4bytes 2^31 -1
    let local_x:i32  = world_x.rem_euclid(side as i32);
    let local_y:i32  = world_y.rem_euclid(side as i32);
    let local_z:i32  = world_z.rem_euclid(side as i32);
        println!("Local x = {}, y = {}, z = {}", local_x, local_y, local_z);
    
    // bool usado para verificar se a comparacao eh true ou false
    // as usado como estreitamento, side de valor 32 tipo usize 8bytes 2^64 cabe em i32 4bytes 2^31 -1
    let axis_x:bool = chunk_x * side as i32 + local_x == world_x;
    let axis_y:bool = chunk_y * side as i32 + local_y == world_y;
    let axis_z:bool = chunk_z * side as i32 + local_z == world_z;
        println!("A reconstrução da coordenada global x a partir do chunk x e local x é válida? {}", axis_x);
        println!("A reconstrução da coordenada global y a partir do chunk y e local y é válida? {}", axis_y);
        println!("A reconstrução da coordenada global z a partir do chunk z e local z é válida? {}", axis_z);

// item D
    // usize usado como tipo padrao de indice
    // as usado como alargamento e mudanca de sinal, valores locais xyz sao todos positivos pq sao resultados do rem euclid e menores que 32 entao eh uma conversao segura e a parte positiva de i32 0 a 2.147.483.647 cabe dentro de usize 2^64
    let index_local: usize = local_x as usize + local_y as usize * side + local_z as usize * side * side;
        println!("O indice local vale {:?}", index_local);
    let bool_index_local: bool = index_local < side * side * side;
        println!("O indice local cabe dentro do tamanho max do chunk? {}", bool_index_local);
// item E
    // usize usado para demonstrar a conversao falha do world_x de valor negativo
    // as de alargamento e troca de sinal usado de forma desleixada pois i32 para usize nao eh uma conversao segura 
    // nao ha garantia do valor world x sempre ser positiva entao a conversao pode gerar um numero diferente do esperado
    // tanto em debug e realese o valor vai ser gerado de forma equivocada pois nao ha mecanismos de checagem do compilador, como em checagens fora do indice
    // ao tentar indexar o valor gigante no array de voxel teria um prblema de panic tanto em debug quanto em realese pois o array de voxel tem tamanho maximo side^3 -1 e o indice falho ultrapassa ele
    let fail_conversion: usize = world_x as usize;
    println!("índice se a conversão viesse antes da garantia = {}", fail_conversion);

// item F
    // f32 usado para comunicacao padrao gpu
    // as cast explcito usado para mudanca de tipo i para f , pode gerar erros em valores acima de 16.777.216 2^24 pelo limite de represtacao exata de f32
    // Rust irá arredondar o valor para o ponto flutuante representável mais próximo
    let far_x_f32: f32 = far_x as f32;
    // f32 usado para comunicacao padrao gpu
    // as cast esplcito usado no mesmo cenario de far_x e far_x_f32, mas para cam nao ha perda de dados e a conversao gera o resultado exato esperado pois o valor original esta abaixo de 16.777.216 2^24
    let cam_far_x_f32: f32 = cam_far_x as f32;
        println!("O valor original de far x eh {} e o valor apos conversao as f32 eh {}", far_x, far_x_f32);
        println!("O valor original de cam far x eh {} e o valor apos conversao as f32 eh {}", cam_far_x, cam_far_x_f32);

    let delta_farx_camfarx_f32: f32 = cam_far_x_f32 - far_x_f32;
        println!("O resultado errado obtido convertendo antes de subtrair eh de {}", delta_farx_camfarx_f32);
    let delta_farx_camfarx: i32 = cam_far_x - far_x;
        println!("O resultado correto obtido subtraindo antes da conversao eh de {}", delta_farx_camfarx);
    let delta_x_correto: f32 = delta_farx_camfarx as f32;
        println!("O delta x das posicoes de modo correto (subtracao -> conversao) tem o valor verdadeiro final igual a {}", delta_x_correto);
    // == aqui é correto por que não há erro acumulado de arredondamento de operacoes anteriores por que a conversão final delta_farx_camfarx as f32 (no caminho certo) não introduz nenhum arredondamento novo
        println!("O valores entre o delta correto e o delta errado sao os mesmos? {}", delta_x_correto == delta_farx_camfarx_f32);
        println!("A diferenca entre o valor correto e o valor errado eh de {} unidade", delta_x_correto - delta_farx_camfarx_f32);

// item g
    let is_solid:bool = id_broken != id_air;
    // Usa `From`. `u8` -> `usize` é garantido (lossless) porque `usize` tem no mínimo 16 bits (maior que os 8 bits do u8).
    // O oposto não vale para `usize` -> `i32` (risco de estouro de tamanho) nem para `i32` -> `usize` (risco de valores negativos).
    let id_broken_u: usize = usize::from(id_broken);
    let broken_voxel_tough: u8 = tough[id_broken_u];
        println!("Bloco eh solido? {}", is_solid);
        println!("a dureza do bloco cujo ID é {} é {}", id_broken, broken_voxel_tough);

// item h
    let can_break_voxel: bool = bool_index_local && is_solid;
        println!("O voxel pode ser quebrado? {}", can_break_voxel);

// resposta 1:
    // short circuit eh quando uma expressao booleana eh computada da esquerda para a direita, no caso de && se o primeiro termo for false (o que torna && falso), os outros termos nao sao computados tambem para evitar acesso fora de indes e preservar trabalho computacional com contas desnecessarias
// resposta 2
    // bool_index_local previne panic em voxels[index_local] (limite side³); tough[id_broken_u] (tamanho 4) é seguro por design de IDs válidos de bloco.    

    


}