use std::io::{self, Write};

pub fn get_user_input() -> String {
    let mut input = String::new();
    io::stdout()
        .flush()
        .expect("Не удалось очистить буфер");

    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    input.trim().to_string()
}