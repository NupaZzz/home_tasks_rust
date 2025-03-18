use crate::player::Player;

pub struct Shop {
    pub name: String,
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

impl Shop {
    pub fn new() -> Self {
        Self {
            name: String::from("Магазин пчёлка"),
            items: vec![
                Item {
                    name: String::from("Деревянный меч"),
                    price: 10,
                    description: String::from("Меч - оружие ближнего боя"),
                    effect: String::from("+2 урона"),
                    durability: 100,
                    attack: 2,
                },
            ],
        }
    }

    pub fn show_items(&self) {
        for item in &self.items {
            println!("{}", item.name);
            println!("  Цена: {}", item.price);
            println!("  Описание: {}", item.description);
            println!("  Эффект: {}", item.effect);
            println!("  Прочность: {}", item.durability);
            println!("--------------------------------");
        }
    }

    pub fn sell_item_to_player(&mut self, item_name: &str, player: &mut Player) {
        for item in &self.items {
            if item.name == item_name {
                if player.inventory[0].gold >= item.price {
                    player.inventory[0].gold -= item.price;
                    player.inventory[0].items.push(item.clone().into());
                    println!(
                        "Вы купили '{}'",
                        item.name
                    );
                } else {
                    println!(
                        "У вас недостаточно золота для покупки '{}'",
                        item.name
                    );
                }

            }
        }
    }
}