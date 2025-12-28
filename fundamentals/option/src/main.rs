fn busca_nome(id: u64) -> Option<String> {
    if id == 1 {
        Some("Sérgio".into())
    } else {
        None
    }
}

fn main() {
    match busca_nome(1) {
        Some(nome) => println!("Nome: {}", nome),
        None => println!("Não encontrado"),
    }
}
