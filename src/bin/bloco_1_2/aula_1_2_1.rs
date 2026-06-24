fn main() {
    let lado: i32 = 32;
    let chunk_x: i32 = 5;
    let mut cam_x: i32 = 0;

    println!("lado: {}", lado);
    println!("chunk_x: {}", chunk_x);
    println!("cam_x inicial: {}", cam_x);

    cam_x = 10;
    println!("cam_x depois da primeira reatribuicao: {}", cam_x);

    cam_x = 20;
    println!("cam_x depois da segunda reatribuicao: {}", cam_x);
}
