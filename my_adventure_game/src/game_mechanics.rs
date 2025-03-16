use crate::enemy::Enemy;
use crate::player::Player;

pub fn enemy_spawn() -> Enemy {
    let enemy = Enemy::new();
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
        let mut enemy = enemy_spawn();
        game_loop(player, &mut enemy);
    }
}