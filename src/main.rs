use std::io::{self, Write};

fn flush() {
    io::stdout().flush().unwrap();
}

fn sum(num1: f32, num2: f32) -> f32 {
    num1 + num2
}

fn sub(num1: f32, num2: f32) -> f32 {
    num1 - num2
}

fn prod(num1: f32, num2: f32) -> f32 {
    num1 * num2
}

fn div(num1: f32, num2: f32) -> f32 {
    (num1 / num2) as f32
}

fn main() {
    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operation = String::new();
        let mut new_operation = String::new();

        println!("===== CALCULADORA EM RUST =====");

        print!("Insira o número 1: ");
        flush();
        io::stdin().read_line(&mut num1).expect("Falha na leitura!");

        print!("Insira o número 2: ");
        flush();
        io::stdin().read_line(&mut num2).expect("Falha na leitura!");

        println!(
            "Qual operação deseja realizar?\n
    - Somar\n
    - Subtrair\n
    - Multiplicar\n
    - Dividir"
        );

        print!("Sua escolha: ");
        flush();
        io::stdin()
            .read_line(&mut operation)
            .expect("Falha na leitura");

        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();

        if operation.trim().eq_ignore_ascii_case("somar") {
            println!("{} + {} = {}\n", num1, num2, sum(num1, num2));
        } else if operation.trim().eq_ignore_ascii_case("subtrair") {
            println!("{} - {} = {}\n", num1, num2, sub(num1, num2));
        } else if operation.trim().eq_ignore_ascii_case("multiplicar") {
            println!("{} * {} = {}\n", num1, num2, prod(num1, num2));
        } else if operation.trim().eq_ignore_ascii_case("dividir") {
            if num2 as i32 == 0 {
                println!("Você não pode dividir por ZERO!");
            } else {
                println!("{} / {} = {:.2}\n", num1, num2, div(num1, num2));
            }
        } else {
            println!("Operação inválida!");
        }

        print!("Deseja realizar outro cálculo? R: ");
        flush();
        io::stdin()
            .read_line(&mut new_operation)
            .expect("Falha na leitura");

        if new_operation.trim().eq_ignore_ascii_case("não") {
            break;
        }
    }
}

