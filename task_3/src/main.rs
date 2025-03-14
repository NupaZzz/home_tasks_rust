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

fn five_numbers() {
    let mut count = 0;
    let mut vec: Vec<i32> = Vec::new();
    loop {
        print!("Введите число: ");
        match user_input().parse::<i32>() {
            Ok(num) => {
                vec.push(num);
                count += 1;
                if count == 5 {
                    println!("Вектор: {:?}", vec);
                    break;
                }
            },
            Err(_) => {
                println!("Ошибка: Введите число!");
            }
        }
    }
}

fn main() {
    print!("Выберите задание: ");
    match user_input().trim().parse::<i32>() {
        Ok(input) => {
            match input {
                1 => {
                    println!("Вы выбрали задание: {input}");
                    five_numbers();
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