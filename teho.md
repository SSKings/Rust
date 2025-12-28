Tabela de Operadores de Tratamento de Erro em Rust

Operador / Estrutura	Descrição	Exemplo	Comportamento
? (try operator)	    Propaga o erro automaticamente. Se Ok, continua; se Err, retorna o erro da função atual.	rust fn dividir(a: i32, b: i32) -> Result<i32, String> { if b==0 { Err("divisão por zero".into()) } else { Ok(a/b) } } fn processa() -> Result<(), String> { let res = dividir(10,2)?; println!("{}", res); Ok(()) }	Early return automático se houver erro.
unwrap	Extrai o valor de Ok. Se for Err, o programa panic e termina.	rust let res = dividir(10,2).unwrap(); println!("{}", res);	Útil para protótipos, mas perigoso em produção.
expect	Igual ao unwrap, mas permite adicionar uma mensagem personalizada em caso de panic.	rust let res = dividir(10,0).expect("Erro ao dividir!");	Panica com a mensagem fornecida se houver erro.
match	Forma explícita de lidar com Result ou Option. Permite tratar Ok e Err separadamente.	rust match dividir(10,0) { Ok(v) => println!("Resultado: {}", v), Err(e) => println!("Erro: {}", e), }	Total controle sobre o que fazer em caso de erro ou sucesso.
Resumo rápido

? → Propaga o erro automaticamente (early return).

unwrap → Extrai valor, panica se erro.

expect → Igual a unwrap, mas com mensagem personalizada.

match → Controle total, tratamento explícito de Ok e Err.

Se você quiser, posso fazer um diagrama visual mostrando o fluxo de execução de cada um (?, unwrap, expect, match) para ver como o erro percorre o código, que ajuda muito a fixar o conceito.

Quer que eu faça isso?

fn dividir(a: i32, b: i32) -> Result<i32, String> { 
    if b==0 { 
        Err("divisão por zero".into()) 
    } 
    else { 
        Ok(a/b) 
    }

} 
    
fn processa() -> Result<(), String> { 
    let res = dividir(10,2)?; 
    println!("{}", res); 
    Ok(()) 
}