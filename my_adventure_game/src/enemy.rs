use crate::player::Player;
use std::time::Duration;
use std::thread::sleep;

pub struct Enemy {
    pub name: String,
    pub health: i32,
    pub damage: i32,
    pub gold_reward: u32,
    pub exp_reward: i32,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            name: String::from("Вражина"),
            health: 5,
            damage: 2,
            gold_reward: 1,
            exp_reward: 10,
        }
    }

    pub fn attack(&self, player: &mut Player) {
        player.current_health -= self.damage;
        sleep(Duration::from_secs_f64(0.5));
        
    }

    pub fn show_stats(&self) -> String {
        format!("{}: {} здоровья", self.name, self.health)
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}