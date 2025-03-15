use std::io::{self, Write};

fn user_input() -> Result<i32, String>{
    let mut input = String::new();
    io::stdout()
        .flush()
        .expect("Ошибка при очистке буфера");
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка при чтении строки");

    input.trim().parse::<i32>().map_err(|err| err.to_string())
}

fn task_1(vec: Vec<i32>) -> i32 {
    vec.iter().sum()
}

fn task_1_vector() {
    let vec = vec![10,40,30,20,50];
    let result = task_1(vec);
    println!("Сумма магических чисел: {}", result);
    if result > 100 {
        println!("Поздравляем, сумма магических чисел превышает 100!");
    }
}

fn task_2(vec: Vec<i32>) -> Vec<i32> {
    let is_even = |vec: &i32| vec % 2 == 0;
    vec.into_iter().filter(is_even).collect()
}

fn task_2_vector() {
    let vec = vec![1,5,4,6,3];
    let result = task_2(vec);
    println!("Четные числа: {:?}", result);
}

fn main() {
    println!("Задание 1. Сокровища Вектора.");
    println!("Задание 2. Лабиринт замыканий.");
    print!("Выберите задание: ");
    match user_input() {
        Ok(num) => {
            match num {
                1 => {
                    println!("Вы выбрали задание 1. Сокровища Вектора.");
                    task_1_vector();
                }
                2 => {
                    println!("Вы выбрали задание 2. Лабиринт замыканий.");
                    task_2_vector();
                }
                _ => {
                    println!("Такого задания не существует");
                }
            }
        }
        Err(err) => println!("Ошибка: {}", err),
    }

}
