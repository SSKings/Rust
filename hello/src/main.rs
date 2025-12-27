fn main(){
    let mut x = 10;
    print!("{}\n",x);
    x = 20;
    println!("{}", x);
    println!("Hello wolrd!");
    let result = soma(x, 10);
    println!("{}", result);
}

fn soma(x:i32, y:i32) -> i32 {
    x + y
}
