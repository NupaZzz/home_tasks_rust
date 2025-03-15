use std::io::{self, Write};

fn user_input() -> String{
    let mut input = String::new();
    io::stdout()
        .flush()
        .expect("Не удалось очистить буфер");
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    input.trim().to_string()
}

fn main() {
    println!("Приветсвую! Как вас зовут?");
    print!("Введите ваше имя: ");
    let name = user_input();
    println!("Привет, {name}! Добро пожаловать в мир Rust!");
}
