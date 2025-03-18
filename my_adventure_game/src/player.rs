use crate::enemy::Enemy;
use std::time::Duration;
use std::thread::sleep;
use rand::random;
use crate::shop::Item as ShopItem;

pub struct Player {
    pub name: String,
    pub current_health: i32,
    pub health_max: i32,
    pub current_mana: u32,
    pub mana_max: u32,
    pub damage: u32,
    pub level: u32,
    pub current_exp: u32,
    pub exp_to_next_level: u32,
    pub inventory: Vec<Inventory>,
}

pub struct Inventory {
    pub gold: u32,
    pub items: Vec<Item>,
}

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub description: String,
    pub effect: String,
    pub durability: u32,
    pub attack: u32,
}

impl From<ShopItem> for Item {
    fn from(shop_item: ShopItem) -> Self {
        Item {
            name: shop_item.name,
            price: shop_item.price,
            description: shop_item.description,
            effect: shop_item.effect,
            durability: shop_item.durability,
            attack: shop_item.attack,
        }
    }
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            current_health: 100,
            health_max: 100,
            current_mana: 100,
            mana_max: 100,
            damage: 1,
            current_exp: 0,
            exp_to_next_level: 100,
            level: 1,
            inventory: vec![Inventory { gold: 100, items: vec![]}],
        }
    }

    pub fn attack(&mut self, enemy: &mut Enemy) -> String {
        enemy.health -= self.damage as i32;
        sleep(Duration::from_secs_f64(0.5));
        format!("Вы атаковали врага и нанесли ему {}", self.damage)
    }

    pub fn show_stats(&self) -> String {
        format!(
            "{}: {} Уровень ({} здоровья, {} маны), {} урона",
            self.name, self.level, self.current_health, self.current_mana, self.damage
        )
    }

    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }

    pub fn level_up(&mut self) -> String {
        self.level += 1;
        self.exp_to_next_level *= 2;
        self.health_max += 10;
        self.mana_max += 5;
        self.damage += 1;
        self.current_health = self.health_max;
        self.current_mana = self.mana_max;
        format!(
            "Поздравляем! Вы достигли {} уровня! Ваши характеристики повышены!",
            self.level
        )
    }

    pub fn add_exp(&mut self, exp: u32) {
        self.current_exp += exp;
        if self.current_exp >= self.exp_to_next_level {
            println!(
                "{}",
                self.level_up()
            );
        }
    }

    pub fn add_gold(&mut self, gold: u32) {
        self.inventory[0].gold += gold;
    }
    
    pub fn rob_gold(&mut self) {
        let amount = self.inventory[0].gold * 50/100;
        self.inventory[0].gold -= amount;
    }

    pub fn restore_mana(&mut self) {
        self.current_mana = self.mana_max;
    }

    pub fn restore_health(&mut self) {
        self.current_health = self.health_max;
    }

    pub fn player_rest(&mut self) -> String {
        println!(
            "Вы отдыхаете..."
        );
        sleep(Duration::from_secs_f64(2.0));
        let chance = random::<f32>();
        if chance < 0.1 {
            println!(
                "Пока вы спали вас ограбили! Вы потеряли {} золота",
                self.inventory[0].gold * 50/100
            );
            self.rob_gold();
        }
        self.restore_health();
        self.restore_mana();
        format!(
            "Вы отдохнули и восстановили здоровье и ману."
        )
    }

    pub fn show_inventory(&self) -> String {
        format!(
            "Инвентарь: {} золота.\nПредметы: {}",
            self.inventory[0].gold,
            self.inventory[0].items.iter().map(|item| item.name.clone()).collect::<Vec<_>>().join(", ")
        )
    }

    pub fn equip_item(&mut self, item_name: &str) {
        for item in &mut self.inventory[0].items {
            if item_name == item.name {
                if item.durability > 0 {
                    self.damage += item.attack;
                    item.durability -= 1;
                    println!("Вы экипировали {} и увеличили урон на {}", item.name, item.attack);
                } else {
                    println!("{} сломан", item.name);
                }
            }
        }
    }
}

