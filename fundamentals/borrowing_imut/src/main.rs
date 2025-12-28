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
/// O owner é quem é responsável por limpar a memória.
/// Passar &s para uma função não muda o dono. O dono continua sendo s dentro de main.
/// Rust garante que enquanto houver referências ativas, você não pode modificar ou liberar 
/// o dono de forma insegura.
