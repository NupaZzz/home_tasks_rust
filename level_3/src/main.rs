struct Hero {
    name: String,
    level: u8,
    health: i32,
    strength: u8,
    agility: u8
}

impl Hero {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            level: 1,
            health: 100,
            strength: 10,
            agility: 5,
        }
    }
    
    fn character_description(&self) -> String {
        format!("Герой {} (Уровень: {}, Здоровье: {}, Сила: {}, Ловкость {}", self.name, self.level, self.health, self.strength, self.agility)
    }

    fn character_story(&self) -> String {
        format!("Герой {} - это герой, который был создан для защиты своего королевства от злых монстров. Он обладает силой и ловкостью, чтобы защитить своих людей.", self.name)
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.health += 20;
    }
}

fn main() {
    let mut hero = Hero::new("Kekes");
    println!("{}", hero.character_description());
    println!("{}", hero.character_story());
    hero.level_up();
    println!("{}", hero.character_description());
}
