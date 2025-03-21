use crate::enemy::Enemy;
use crate::player::Player;
use crate::user_input::get_user_input;
use crate::shop::Shop;

pub fn show_stats(player: &Player, enemy: &Enemy) {
    println!("{}", player.show_stats());
    println!("{}", enemy.show_stats());
}

pub fn game_loop(player: &mut Player, enemy: &mut Enemy) {
    loop {
        println!("{}", enemy.attack(player));
        if !player.is_alive() {
            println!("Вы погибли. Игра окончена.");
            std::process::exit(0);
        }
        println!("{}",player.attack(enemy));
        if !enemy.is_alive() {
            println!("Вы победили врага и получили {} золота и {} опыта.",
                     enemy.gold_reward, enemy.exp_reward);
            player.add_gold(enemy.gold_reward);
            player.add_exp(enemy.exp_reward);
            break;
        }
    }
    show_stats(player, enemy);
}

pub fn game_start(player: &mut Player, enemy: &mut Enemy, shop: &mut Shop) {
    loop {
        println!("1.Пойти в приключение");
        println!("2.Отдохнуть");
        println!("3.Показать статистику");
        println!("4.Посмотреть инвентарь");
        println!("5.Пойти в магазин");
        println!("0.Выйти из игры");
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
                println!("{}", player.player_rest());
            }
            Ok(3) => {
                println!("{}", player.show_stats());
            }
            Ok(4) => {
                println!("{}", player.show_inventory());
                print!("Введите название предмета для экипировки: ");
                let equip_item = get_user_input();
                player.equip_item(&equip_item);
            }
            Ok(5) =>{
                println!("Добро пожаловать в '{}'", shop.name);
                shop.show_items();
                print!("Введите название предмета для покупки: ");
                let item_name = get_user_input();
                shop.sell_item_to_player(&item_name, player);
            }

            Ok(0) => {
                println!("Выход из игры...");
                std::process::exit(0);
            }
            _ => {
                println!("Неверный выбор. Пожалуйста, введите 1 или 2.");
            }
        }
    }
}