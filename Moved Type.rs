fn main() {
    let a = String::from("Rust");
    let b = a; // moves value of 'a' to 'b'
    eprintln!("a:{} , b:{}", a, b); // Error use of moved value 'a'
}
