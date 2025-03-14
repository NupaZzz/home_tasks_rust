use std::io::{self, Write};
use rand::Rng;

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
            let result = input;
            let mut factorial = 1;
            let mut vector = vec![];
            while input > 0 {
                factorial *= input;
                vector.push(input);
                input -= 1;
            }
            let vector = vector.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" * ");
            println!("{}! = {} = {}", result, vector, factorial);
        }
        Err(_) => {
            println!("Ошибка: Введите число!");
        }
    }
}

fn guess_number() {
    println!("Загадано число от 1 до 100, попробуйте его угадать!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 0;

    loop {
        count += 1;
        print! ("Попытка {count}, введите число: ");
        match user_input()
            .trim()
            .parse::<i32>() {
                Ok(guess) => {
                    if guess == secret_number {
                        println!("Поздравляю! Вы угадали число {secret_number} за {count} попыток!");
                        break;
                    } else if guess < secret_number {
                        println!("Слишком маленькое число!");
                    } else if guess > secret_number {
                        println!("Слишком большое число!");
                    }
                }
                Err(_) => {
                    println!("Ошибка: Введите число!");
                }
            }
        if count == 5 {
            println!("Вы исчерпали все попытки! Загаданое число было {secret_number}!");
            break;
        }
    }
}

fn main() {
    print!("Введите число от 1 до 5 что бы выбрать задание: ");
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
        },
        5 => {
            println!("Вы выбрали задание: {input}!");
            guess_number();
        },
        _ => println!("Ошибка: Введите число от 1 до 5!"),
    }
}