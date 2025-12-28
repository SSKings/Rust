fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Divisão por zero".into())
    } else {
        Ok(a / b)
    }
}

fn main() {

    match dividir(10, 0) {
        Ok(v) => println!("Resultado: {}", v),
        Err(e) => println!("Erro: {}", e),
    }

    match dividir(10, 2) {
        Ok(v) => println!("Resultado: {}", v),
        Err(e) => println!("Erro: {}", e),
    }
}
