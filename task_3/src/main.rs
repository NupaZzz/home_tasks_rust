use std::{
    io::{self, Write},
    collections::HashMap
};

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

fn student_info() {
    let mut map = HashMap::new();
    print!("Введите имя студента: ");
    let input = user_input();
    map.insert("Володя", 1);
    map.insert("Саня", 2);
    map.insert("Кирилл", 3);
    map.insert("Андрей", 4);
    map.insert("Дима", 5);
    map.insert("Саша", 6);
    map.insert("Миша", 7);
    map.insert("Женя", 8);

    if let Some(value) = map.get(input.as_str()) {
        println!("Студент: {input}, оценка: {value}");
    } else {
        println!("Ошибка: Студент не найден!");
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
                    student_info();
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