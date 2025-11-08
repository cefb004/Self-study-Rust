use std::io;

fn main() {
    println!("Conversor de Temperatura");
    println!("Digite a temperatura em Celsius:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro na leitura");

    let celsius: f64 = input.trim().parse().expect("Digite um número válido!");
    let fahrenheit = celsius_to_fahrenheit(celsius);

    println!("{celsius}°C equivalem a {fahrenheit:.2}°F");
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

