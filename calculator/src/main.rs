use std::io::{self, Write};

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn sub(a: f64, b: f64) -> f64 {
    a - b
}

fn mul(a: f64, b: f64) -> f64 {
    a * b
}

fn div(a: f64, b: f64) -> f64 {
    a / b
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Не удалось сбросить стандартный вывод");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");
    input.trim().to_string()
}

fn main() {
    println!("Калькулятор");

    // Чтение первого числа
    let number_one: f64 = read_input("Введите первое число: ")
        .parse()
        .expect("Неверный формат числа!");

    // Чтение второго числа
    let number_two: f64 = read_input("Введите второе число: ")
        .parse()
        .expect("Неверный формат числа!");

    // Чтение операции
    let op_input = read_input("Введите операцию (+, -, *, /): ");
    if op_input.len() != 1 {
        println!("Неверная операция");
        return;
    }
    let operation = op_input.chars().next().unwrap();

    // Выбор операции через match
    match operation {
        '+' => println!("{} + {} = {}", number_one, number_two, add(number_one, number_two)),
        '-' => println!("{} - {} = {}", number_one, number_two, sub(number_one, number_two)),
        '*' => println!("{} * {} = {}", number_one, number_two, mul(number_one, number_two)),
        '/' => {
            if number_two == 0.0 {
                println!("На ноль делить нельзя!");
            } else {
                println!("{} / {} = {}", number_one, number_two, div(number_one, number_two))
            }
        }
        _ => println!("Неверная операция"),
    }
}
