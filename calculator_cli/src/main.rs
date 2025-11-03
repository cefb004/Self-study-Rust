use std::io;

fn main() {
    println!("Rust CLI Calculator");
    println!("=========================");

    // Lê o primeiro número
    let mut first = String::new();
    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut first).expect("Erro na leitura");
    let first: f64 = first.trim().parse().expect("Por favor, digite um número válido");

    // Lê o segundo número
    let mut second = String::new();
    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut second).expect("Erro na leitura");
    let second: f64 = second.trim().parse().expect("Por favor, digite um número válido");

    // Lê a operação
    let mut op = String::new();
    println!("Escolha uma operação (+, -, *, /): ");
    io::stdin().read_line(&mut op).expect("Erro na leitura");
    let op = op.trim();

    // Calcula o resultado
    let result = match op {
        "+" => first + second,
        "-" => first - second,
        "*" => first * second,
        "/" => {
            if second == 0.0 {
                println!("❌ Erro: divisão por zero não permitida!");
                return;
            } else {
                first / second
            }
        }
        _ => {
            println!("❌ Operação inválida!");
            return;
        }
    };

    // Exibe o resultado formatado
    println!("=========================");
    println!("Resultado: {} {} {} = {}", first, op, second, result);
    println!("=========================");
}

