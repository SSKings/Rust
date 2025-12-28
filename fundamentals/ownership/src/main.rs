/// Em Rust, cada valor tem um único dono.

fn main() {
    let s1 = String::from("Olá Rust");
    let s2 = s1;

    // println!("{}", s1); // ERRO: valor movido
    println!("{}", s2);
}

/** s1 era dono da String
Ao atribuir para s2, a propriedade foi movida
s1 deixou de ser válido **/

/// Regra-chave: não existem cópias implícitas de dados alocados no heap.