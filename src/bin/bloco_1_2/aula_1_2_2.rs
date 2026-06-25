fn main() {
    let chunk_x: i32 = -3;
    let chunk_y: i32 = 2;
    let chunk_z: i32 = 7;
    // nao usar unsigned para coordenadas de voxel, pois o mundo pode ter coordenadas negativas, e unsigned nao permite valores negativos
    // sem mut pois as coordenadas de chunk não mudam durante a execução do programa, apenas o player que muda de chunk
        println!("chunk_x: {}", chunk_x);
        println!("chunk_y: {}", chunk_y);
        println!("chunk_z: {}", chunk_z);

    let chunk_side: usize = 32;
    // usize usado pois o item pedia o mesmo tipo usado para indices
    // sem mut pois o tamanho do chunk não muda durante a execução do programa
    // proibido #[repr(C)] pois a compatibilidade entre sistemas 32 e 64 bits nao é garantida
        println!("chunk_side: {}", chunk_side);

    let voxel_coord: (u8, u8, u8) = (10, 20, 30);
    // u8 usado pois as coordenadas internas do chunk são sempre menores que 32, e positivas
    // sem mut pois as coordenadas do voxel não mudam durante a execução do programa
        println!("voxel_coord: {:?}", voxel_coord);


    let mut voxel_id: u8 = 0;
        println!("voxel_id: {}", voxel_id);
    voxel_id = 7;
    // u8 usado pois eh uma variavel hot path que sera usada para indexar um vetor de 256 elementos positivos
    //com mut pois o id do voxel muda durante a execução do programa, conforme o player interage com o mundo
        println!("voxel_id: {}", voxel_id);


    let mut voxel_vertex: (f32, f32, f32) = (0.0, 0.0, 0.0);
        println!("voxel_vertex: {:?}", voxel_vertex);
    voxel_vertex = (5.0, 0.0, 30.0);
    // f32 usado pois as coordenadas do vértice podem ser valores fracionários
    // com mut pois as coordenadas do vértice mudam durante a execução do programa, conforme o player interage com os voxels e malhas sao recalculadas
        println!("voxel_vertex: {:?}", voxel_vertex);


    let voxel_light: u8 = 250;
        println!("voxel_light: {}", voxel_light);
    // u8 usado pois a intensidade da luz é um valor positivo entre 0 e 255
    // com mut pois a intensidade da luz muda durante a execução do programa, conforme o player interage com os voxels e a luz é recalculada
    let voxel_light_saturating: u8 = voxel_light.saturating_add(10);
    // saturating_add usado para evitar overflow, caso a soma ultrapasse 255, o valor será limitado a 255
        println!("voxel_light_saturating: {}", voxel_light_saturating);
    let voxel_light_wrapping: u8 = voxel_light.wrapping_add(10);
    // wrapping_add usado para permitir overflow, caso a soma ultrapasse 255, o valor será reiniciado a partir de 0
        println!("voxel_light_wrapping: {}", voxel_light_wrapping);
}
