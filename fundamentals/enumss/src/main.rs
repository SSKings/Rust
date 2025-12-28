enum Status {
    Ativo,
    Inativo,
}

fn verifica(status: Status) {
    match status {
        Status::Ativo => println!("Usuário ativo"),
        Status::Inativo => println!("Usuário inativo"),
    }
}
