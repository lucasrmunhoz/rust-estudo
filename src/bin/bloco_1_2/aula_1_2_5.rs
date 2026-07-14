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
    let longe_x: i32 = 16_777_217;
    // i32: Mantém a consistência de tipo com longe_x para permitir comparações e cálculos diretos, escolhido deliberadamente para testar a fronteira de 2²⁴ no cast para f32 
    let cam_longe_x: i32 = 16_777_000;
        println!("side {}, id air {}, id broken {}, tough {:?}, cam x {}, longe x {}, cam longe x {}"
        , side, id_air, id_broken, tough, cam_x, longe_x, cam_longe_x);
//item B
    // i32 para representar chunks em posicoes inteiras positivas e negativas, a conta correta usa div euclid para o calculo correto do chunk
    // as justificado para tranformar side que eh usize pequeno para i32 para a divisao com world_x i32
    let chunk_x: i32 = world_x / side as i32; // esperado -3 com a conta correta div euclid
    // u8 seria o correto pois coordenadas locais sao sempre 0 a 31 quando usando rem euclid corretamente
    // i8 usado pois o teste para mostrar o calculo errado gera uma coord local negativa impossivel
    // as usado na expressao para trandormar world_x que eh pequeno e i32 para i8 e side que eh usize pequeno para i8, os dois no mesmo tipo para que % funcione corretamente
    let local_x: i8  = world_x as i8 % side as i8; // esperado 26 com rem.euclid
        println!("conta errada do chunk x = {} e local x = {}", chunk_x, local_x);
    // chunk_x aponta para o chunk errado pois / em coordenadas negativas tem o arrendondamento para o valor inteiro proximo de 0 (-2,18 para -2)
    // local_x aponta para uma coordenada imposivel pois coords local vao de 0 a 31 e % eh o resto de / ((-2 . 32) -6 = -70)
    // o sintoma no jogo seria a busca do chunk errado afetando os voxel do chunk -2 e a busca da coord local -6 causa panic por busca fora do index
    // antes do jogo travar o jogador veria bugs visuais nos voxels do chunk -2


}