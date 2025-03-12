use std::io::{self, Write};

fn user_input() -> String {
    let mut input = String::new();
    io::stdout()
        .flush()
        .expect("Не удалось сбросить стандартный вывод");

    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    input
}

fn even_odd_number() {
    print!("Введите число что бы узнать, чётное ли оно: ");
    let input: f64 = user_input()
        .trim()
        .parse()
        .unwrap();

    if input % 2.0 == 0.0 {
        println!("Число чётное!");
    } else {
        println!("Число не чётное!");
    }
}

fn day_of_week() {
    print!("Введите число от 1 до 7 что бы узнать день недели: ");
    let input: i32 = user_input()
        .trim()
        .parse()
        .unwrap();

    match input {
        1 => println!("Понедельник"),
        2 => println!("Вторник"),
        3 => println!("Среда"),
        4 => println!("Четверг"),
        5 => println!("Пятница"),
        6 => println!("Суббота"),
        7 => println!("Воскресенье"),
        _ => println!("Ошибка: Введите число от 1 до 7!"),
    }
}

fn output_numbers() {
    print!("Введите число: ");
    match user_input().trim().parse() {
        Ok(input) => {
            for i in 1..=input {
                println!("{i}");
            }
        }
        Err(_) => {
            println!("Ошибка: Введите число!");
        }
    }
}

fn factorial() {
    print!("Введите число что бы узнать его факториал: ");
    match user_input().trim().parse::<i32>() {
        Ok(mut input) => {
            if input == 0 {
                println!("0! = 1");
            } else if input < 0 {
                println!("Факториал не существует!");
            }
            while input > 0 {
                println!("{input}");
                input -= 1;
            }
        }
        Err(_) => {
            println!("Ошибка: Введите число!");
        }
    }
}

fn main() {
    print!("Введите число от 1 до 6 что бы выбрать задание: ");
    let input: i32 = user_input()
        .trim()
        .parse()
        .unwrap();

    match input {
        1 => {
            println!("Вы выбрали задание: {input}!");
            even_odd_number()
        },
        2 => {
            println!("Вы выбрали задание: {input}!");
            day_of_week()
        },
        3 => {
            println!("Вы выбрали задание: {input}!");
            output_numbers()
        },
        4 => {
            println!("Вы выбрали задание: {input}!");
            factorial();
        }
        5 => println!("Задание 5"),
        6 => println!("Задание 6"),
        _ => println!("Ошибка: Введите число от 1 до 6!"),
    }
}
