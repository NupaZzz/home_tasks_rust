use crate::enemy::Enemy;
use crate::player::Player;
use crate::user_input::get_user_input;

pub fn enemy_spawn() -> Enemy {
    let enemy = Enemy::new_enemy();
    println!("Вы встретили {}", enemy.name);
    enemy
}

pub fn enemy_spawn_beast() -> Enemy {
    let enemy = Enemy::new_enemy_beast();
    println!("Вы встретили {}", enemy.name);
    enemy
}

pub fn player_attack(player: &mut Player, enemy: &mut Enemy) {
    player.attack(enemy);
}

pub fn enemy_attack(enemy: &mut Enemy, player: &mut Player) {
    enemy.attack(player);
}

pub fn show_stats(player: &Player, enemy: &Enemy) {
    println!("{}", player.show_stats());
    println!("{}", enemy.show_stats());
}

pub fn game_loop(player: &mut Player, enemy: &mut Enemy) {
    loop {
        show_stats(player, enemy);
        enemy_attack(enemy, player);
        player_attack(player, enemy);
        if !enemy.is_alive() {
            println!("Вы победили врага и получили {} золота и {} опыта.", enemy.gold_reward, enemy.exp_reward);
            player.add_gold(enemy.gold_reward);
            player.add_exp(enemy.exp_reward);
            break;
        }
        if !player.is_alive() {
            println!("Вы погибли. Игра окончена.");
            std::process::exit(0);
        }
    }
}

pub fn game_start(player: &mut Player) {
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
                let mut enemy = if player.level >= 2 {
                    enemy_spawn_beast()
                } else {
                    enemy_spawn()
                };
                game_loop(player, &mut enemy);
            }
            Ok(2) => {
                player.player_rest();
                println!("Вы отдохнули и восстановили здоровье и ману.");
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