use std::io::{self, Write};

fn user_input() -> String {
    let mut input = String::new();
    io::stdout()
        .flush()
        .expect("Не удалось очистить буфер");
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    input.trim().to_string()
}

fn five_numers() {
    print!("Введите число 1: ");
    let mut vec: Vec<i32> = Vec::new();
    vec.push(user_input().parse().unwrap());
    print!("Введите число 2: ");
    vec.push(user_input().parse().unwrap());
    print!("Введите число 3: ");
    vec.push(user_input().parse().unwrap());
    print!("Введите число 4: ");
    vec.push(user_input().parse().unwrap());
    print!("Введите число 5: ");
    vec.push(user_input().parse().unwrap());

    for num in &vec {
        println!("Число в векторе: {}", *num);
    }
}

fn main() {
    print!("Выберите задание: ");
    match user_input().trim().parse::<i32>() {
        Ok(input) => {
            match input {
                1 => {
                    println!("Вы выбрали задание: {input}");
                    five_numers();
                },
                2 => {
                    println!("Вы выбрали задание: {input}");
                },
                3 => {
                    println!("Вы выбрали задание: {input}");
                },
                4 => {
                    println!("Вы выбрали задание: {input}");
                },
                _ => {
                    println!("Ошибка: Данного задания не существует!");
                }
            }
        }
        Err(_) => {
            println!("Ошибка: Введите число!");
        }
    }
}