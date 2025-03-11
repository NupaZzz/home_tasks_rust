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

fn calculation(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => add(a, b),
        '-' => sub(a, b),
        '*' => mul(a, b),
        '/' => {
            if b != 0.0 {
                div(a, b)
            } else {
                println!("На ноль делить нельзя!");
                0.0
            }
        }
        _ => panic!("Неверная операция"),
    }
}

fn main() {
    println!("Калькулятор");

    let number_one: f64 = match read_input("Введите первое число: ").parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Ошибка: Введите число!");
            return;
        }
    };

    let number_two: f64 = match read_input("Введите второе число: ").parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Ошибка: Введите число!");
            return;
        }
    };

    // Чтение операции
    let op_input = read_input("Введите операцию (+, -, *, /): ");
    if op_input.len() != 1 {
        println!("Неверная операция");
        return;
    }
    let operation = op_input.chars().next().unwrap();
    let result = calculation(number_one, number_two, operation);
    println!("Результат: {}", result);
}
