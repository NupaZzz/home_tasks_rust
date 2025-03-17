use crate::player::Player;
use std::time::Duration;
use std::thread::sleep;
use rand::Rng;
use crate::game_mechanics::game_loop;

pub struct Enemy {
    pub name: String,
    pub health: i32,
    pub damage: i32,
    pub gold_reward: u32,
    pub exp_reward: i32,
}

impl Enemy {
    pub fn new_enemy() -> Self {
        Self {
            name: String::from("Вражина"),
            health: 5,
            damage: rand::thread_rng().gen_range(1..=2),
            gold_reward: 1,
            exp_reward: 10,
        }
    }

    pub fn new_enemy_beast() -> Self {
        Self {
            name: String::from("Бестия"),
            health: 10,
            damage: rand::thread_rng().gen_range(2..=4),
            gold_reward: 2,
            exp_reward: 20,
        }
    }

    pub fn attack(&self, player: &mut Player) -> String{
        player.current_health -= self.damage;
        sleep(Duration::from_secs_f64(0.5));
        format!("Враг атаковал вас и нанес {} урона", self.damage)
        
    }

    pub fn show_stats(&self) -> String {
        format!("{}: {} здоровья, {} урона", self.name, self.health, self.damage)
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn spawn_and_alert(&self, player: &mut Player) {
        let mut enemy = if player.level >= 2 {
            let enemy = Enemy::new_enemy_beast();
            println!("Вы встретили {}", enemy.name);
            enemy
        } else {
            let enemy = Enemy::new_enemy();
            println!("Вы встретили {}", enemy.name);
            enemy
        };
        game_loop(player, &mut enemy);
    }
}