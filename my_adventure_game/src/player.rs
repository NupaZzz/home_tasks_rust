use crate::enemy::Enemy;
use std::time::Duration;
use std::thread::sleep;

pub struct Player {
    pub name: String,
    pub health: i32,
    pub health_max: i32,
    pub mana: u32,
    pub mana_max: u32,
    pub damage: i32,
    pub gold: u32,
    pub level: i32,
    pub current_exp: i32,
    pub exp_to_next_level: i32,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            health: 100,
            health_max: 100,
            mana: 100,
            mana_max: 100,
            damage: 1,
            gold: 0,
            current_exp: 0,
            exp_to_next_level: 100,
            level: 1,
        }
    }

    pub fn attack(&mut self, enemy: &mut Enemy) {
        enemy.health -= self.damage;
        sleep(Duration::from_secs_f64(0.5));
    }

    pub fn show_stats(&self) -> String {
        format!(
            "{}: {} Уровень ({} здоровья, {} маны), {} золота, {} урона", self.name, self.level, self.health, self.mana, self.gold, self.damage
        )
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.exp_to_next_level *= 2;
        self.health_max += 10;
        self.mana_max += 5;
        self.damage += 1;
        self.health = self.health_max;
        self.mana = self.mana_max;
        println!(
            "Поздравляем! Вы достигли {} уровня! Ваши характеристики повышены!", self.level
        )
    }

    pub fn add_exp(&mut self, exp: i32) {
        self.current_exp += exp;
        if self.current_exp >= self.exp_to_next_level {
            self.level_up();
        }
    }

    pub fn add_gold(&mut self, gold: u32) {
        self.gold += gold;
    }

    pub fn restore_mana(&mut self) {
        self.mana = self.mana_max;
    }

    pub fn restore_health(&mut self) {
        self.health = self.health_max;
    }

    pub fn player_rest(&mut self) {
        self.restore_health();
        self.restore_mana();
    }
}

