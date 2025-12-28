struct Usuario {
    id: u64,
    nome: String,
}

impl Usuario {
    fn novo(id: u64, nome: String) -> Self {
        Self { id, nome }
    }

    fn saudacao(&self) -> String {
        format!("Olá, {}", self.nome)
    }
}

fn main() {
    let user = Usuario::novo(1, "Sérgio".into());
    println!("{}", user.saudacao());
}
