fn main() {
    let a = 3;
    let b = 7;

    let menor: bool = a < b;   // true
    let igual: bool = a == b;
    println!("Menor: {}", menor);     // false  (tipo `bool` inferido)
    println!("Igual: {}", igual);     // false  (tipo `bool` inferido)
}