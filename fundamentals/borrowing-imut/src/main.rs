/// Você pode emprestar uma referência sem transferir ownership.

fn imprime(texto: &String) {
    println!("{}", texto);
}

fn main() {
    let s = String::from("Rust é seguro");
    imprime(&s);
    println!("{}", s);
}
/// O dono continua sendo main