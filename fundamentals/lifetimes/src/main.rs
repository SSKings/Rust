/// Exemplo: Referência inválida sem lifetime explícito

/** 
fn maior_sem_lifetime() -> String {
    let s1 = String::from("Olá");
    let s2 = String::from("Mundo!");
    if s1.len() > s2.len() { &s1 } else { &s2 }
}

fn main(){
     let s = maior_sem_lifetime();
     println!("{}",s);
}
**/

/// Se você tentar compilar, o Rust vai reclamar
 
/** 
Por quê?
s1 e s2 são destruídos ao sair da função, então qualquer referência 
que aponte para eles seria inválida. O Rust impede isso em tempo de compilação. 
**/

///Exemplo com lifetime
fn maior<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let s1 = String::from("Olá");
    let s2 = String::from("Mundo!");
    
    let resultado = maior(&s1, &s2);
    println!("Maior: {}", resultado);
}
