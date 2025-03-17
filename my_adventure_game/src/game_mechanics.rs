use crate::enemy::Enemy;
use crate::player::Player;
use crate::user_input::get_user_input;

pub fn show_stats(player: &Player, enemy: &Enemy) {
    println!("{}", player.show_stats());
    println!("{}", enemy.show_stats());
}

pub fn game_loop(player: &mut Player, enemy: &mut Enemy) {
    loop {
        show_stats(player, enemy);
        enemy.attack(player);
        if !player.is_alive() {
            println!("Вы погибли. Игра окончена.");
            std::process::exit(0);
        }
        player.attack(enemy);
        if !enemy.is_alive() {
            println!("Вы победили врага и получили {} золота и {} опыта.",
                     enemy.gold_reward, enemy.exp_reward);
            player.add_gold(enemy.gold_reward);
            player.add_exp(enemy.exp_reward);
            break;
        }
    }
}

pub fn game_start(player: &mut Player, enemy: &mut Enemy) {
    loop {
        println!("1.Пойти в приключение");
        println!("2.Отдохнуть");
        println!("3.Показать статистику");
        println!("4.Выйти из игры");
        print!("Выберите действие: ");
        let user_choice = get_user_input()
            .trim()
            .parse::<i32>()
            .map_err(|_| "Неверный ввод. Пожалуйста, введите число.");
        match user_choice {
            Ok(1) => {
                enemy.spawn_and_alert(player);
            }
            Ok(2) => {
                player.player_rest();
            }
            Ok(3) => {
                println!("{}", player.show_stats());
            }
            Ok(4) => {
                println!("Выход из игры...");
                std::process::exit(0);
            }
            _ => {
                println!("Неверный выбор. Пожалуйста, введите 1 или 2.");
            }
        }
    }
}