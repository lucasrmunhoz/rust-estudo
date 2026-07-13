fn main() {
    let mundo_x: i32 = -70;
    let chunk: i32 = mundo_x.div_euclid(32);
    let local_x: i32 = mundo_x.rem_euclid(32);
    println!("chunk {} e local x {}", chunk, local_x);


}