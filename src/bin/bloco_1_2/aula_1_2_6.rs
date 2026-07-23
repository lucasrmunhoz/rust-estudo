
use rust_estudo::consts::{CHUNK_SIDE, CHUNK_AREA, CHUNK_VOXEL_COUNT, CHUNK_SIDE_I32, VOXEL_ID_AIR};
const CUBE_FACE_COUNT: usize = 6;
const CHUNK_MESH_FACE_COUNT_NAIVE: usize = CUBE_FACE_COUNT * CHUNK_VOXEL_COUNT;

fn main (){
    println!("A aresta do chunk vale {}", CHUNK_SIDE);
    println!("A area do chunk vale {}", CHUNK_AREA);
    println!("O chunk tem {} voxels", CHUNK_VOXEL_COUNT);
    println!("A aresta do chunk em i32 vale {}", CHUNK_SIDE_I32);
    println!("O voxel com ID ar vale {}", VOXEL_ID_AIR);

    // As três coordenadas locais do voxel que o programa vai escrever;
    let voxel_coord_local_x: usize = 27;
    let voxel_coord_local_y: usize = 15;
    let voxel_coord_local_z: usize = 0;
    // O ID do voxel que o programa vai escrever.
    let voxel_id_default: u8 = 1;

    println!("O cubo tem {} faces", CUBE_FACE_COUNT);
    println!("O meshing completo do chunk vai gerar {} faces", CHUNK_MESH_FACE_COUNT_NAIVE);
    println!("O voxel esta na coordenada local x {}, y {} e z {}", voxel_coord_local_x, voxel_coord_local_y, voxel_coord_local_z);
    println!("O voxel default tem id {}", voxel_id_default);
}
