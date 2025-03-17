use crate::user_input::get_user_input;
use crate::player::Player;
use crate::game_mechanics::game_start;
use crate::enemy::Enemy;

pub fn start_new_game() {
    print!("Введите имя вашего пресонажа: ");
    let player_name = get_user_input();
    let mut player = Player::new(String::from(player_name));
    println!("Добро пожаловать в игру, {}", player.name);
    let mut enemy = Enemy::new_enemy();
    game_start(&mut player, &mut enemy);

}

pub fn exit_game() {
    println!("Выход из игры...");
    std::process::exit(0);
}

pub fn wrong_choice() {
    println!("Неверный выбор. Пожалуйста, введите 1 или 2.");
}