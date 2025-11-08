use std::io;

fn main() {
    println!("Digite um número inteiro:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro de leitura");

    let num: i32 = input.trim().parse().expect("Por favor, digite um número válido");

    if is_even(num) {
        println!("O número {} é par.", num);
    } else {
        println!("O número {} é ímpar.", num);
    }
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

