mod domain;

use domain::usuario::Usuario;

fn main(){

    let usr = Usuario {
        id:10,
        nome:"Sérgio".into(),
    };

    println!("ID:{}\nNome:{}", usr.id, usr.nome);
}