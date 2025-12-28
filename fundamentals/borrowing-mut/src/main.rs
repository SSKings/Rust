fn adiciona(texto: &mut String) {
    texto.push_str(" e rápido");
}

fn main() {
    let mut s = String::from("Rust é seguro");
    adiciona(&mut s);
    println!("{}", s);
}

/** 
Regra crítica:
Apenas uma referência mutável por vez
Ou várias imutáveis, nunca ambos
**/

/* 
Exemplo de erro clássico (e por quê):

fn main() {
    let mut s = String::from("Erro");

    let r1 = &s;
    let r2 = &mut s; // ERRO

    println!("{}", r1);
}

O compilador impede data race em tempo de compilação.
 */