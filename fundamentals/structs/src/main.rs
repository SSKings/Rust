struct Usuario {
    id: u64,
    nome: String,
    idade: u64,
}

fn main() {
    let user = Usuario {
        id: 1,
        nome: String::from("Sérgio"),
        idade: 32,
    };

    println!("Id:{}\nNome:{}\nIdade: {}", user.id, user.nome, user.idade);
}
