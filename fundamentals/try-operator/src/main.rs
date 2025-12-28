/** 
Operador ? em Rust
O operador ? é usado em expressões que retornam Result ou Option. 
Ele propaga erros automaticamente sem precisar escrever match manualmente. **/

fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Divisão por zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn processa() -> Result<(), String> {
    // Aqui usamos o operador `?`:
    // - Se dividir retornar Ok(valor), o valor vai para resultado
    // - Se dividir retornar Err(erro), processa() retorna Err imediatamente
    let resultado = dividir(10, 2)?;  

    println!("Resultado: {}", resultado);

    Ok(())
}

fn main() {
    match processa() {
        Ok(_) => println!("Processamento concluído!"),
        Err(e) => println!("Erro: {}", e),
    }
}
/** 
Como o ? funciona internamente
O operador ? é equivalente a:

let resultado = match dividir(10, 2) {
    Ok(valor) => valor,
    Err(erro) => return Err(erro),
};

Ou seja, faz um early return automático quando há erro.
 */