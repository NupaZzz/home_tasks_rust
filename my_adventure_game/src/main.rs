mod player;
mod user_input;
mod main_menu;
mod game_mechanics;
mod enemy;

use user_input::get_user_input;
use main_menu::{
    exit_game, start_new_game, wrong_choice
};

fn main() {
    println!("Добро пожаловать в игру!");
    loop {
        println!("Меню игры: ");
        println!("  1. Начать новую игру");
        println!("  2. Выйти из игры");
        print!("Выберите действие: ");
        let choice = get_user_input().parse::<u32>().map_err(|_| "Введите число");

        match choice {
            Ok(1) => {
               start_new_game();
            }
            Ok(2) => {
                exit_game();
            }
            _ => wrong_choice(),
        }
    }
}
