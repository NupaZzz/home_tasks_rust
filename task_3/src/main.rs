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

fn join_strings() {
    print!("Введите строку один: ");
    let mut s = user_input();
    print!("Введите строку два: ");
    s.push_str(&user_input());

    println!("{}", s);
}

fn div() -> Result<f64, String> {
    print!("Введите число один: ");
    let a: f64 = user_input()
        .trim()
        .parse::<f64>()
        .map_err(|err| err.to_string())?;

    print!("Введите число два: ");
    let b: f64 = user_input()
        .trim()
        .parse::<f64>()
        .map_err(|err| err.to_string())?;

    if b == 0.0 {
        return Err("Деление на ноль!".to_string());
    }

    Ok(a / b)
}

fn main() {
    println!("1. Задание 1: Ввод пяти чисел для вектора");
    println!("2. Задание 2: Ввод имени студента и вывод оценки");
    println!("3. Задание 3: Сложение двух строк");
    println!("4. Задание 4: Деление двух чисел");
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
                    join_strings();
                },
                4 => {
                    println!("Вы выбрали задание: {input}");
                    match div() {
                        Ok(result) => {
                            println!("Результат: {result}");
                        }
                        Err(err) => {
                            println!("Ошибка: {err}");
                        }
                    }
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