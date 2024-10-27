use serde::{Serialize, Deserialize};
use std::fs::File;
use std::path::PathBuf;
use std::fs;
use std::io::{self, Read};
use rand::Rng;

fn get_summonable_undead(path: &PathBuf) -> io::Result<Vec<String>> {
    let files = fs::read_dir(path)?;

    let mut list = Vec::new();
    for file in files {
        let file = file?;
        let file_name = file.file_name().into_string().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid file name"))?;
        list.push(file_name);
    }
    Ok(list)
}

fn undead_hit(bonus: i8, advantage: u8) -> i8 {
    let mut rng = rand::thread_rng();
    match advantage {
        0 => return rng.gen_range(1..=20) + bonus,
        1 => {
            let r1 = rng.gen_range(1..=20) + bonus;
            let r2 = rng.gen_range(1..=20) + bonus;
            return if r1 > r2 { r1 } else { r2 }
        }, 
        2 => {
            let r1 = rng.gen_range(1..=20) + bonus;
            let r2 = rng.gen_range(1..=20) + bonus;
            return if r1 < r2 { r1 } else { r2 }
        },
        _ => panic!("How the hell did you get here"),
    }
}

fn undead_damage(undead_dice: (u8, u8, i8)) -> u16 {
    let mut rng = rand::thread_rng();
    let number_of_dice = undead_dice.0;
    let value_of_dice = undead_dice.1;
    let modifier = undead_dice.2;
    let mut total: u16 = 0; 
    for _ in 0..number_of_dice{
        total += rng.gen_range(1..=value_of_dice) as u16;
    }
    return total + modifier as u16
}


    fn user_input_yn() -> bool {
    let output: bool;
    loop {
        println!("Enter y/n: ");
        let input = read_input();
        match input.parse::<char>() {
            Ok(value) if 'y' == value || 'n' == value => {
                if value == 'y'{
                    output = true;
                } else {
                    output = false;
                }
                break;
            }
            _ => println!("Answer must be y/n"),
        }
    }
    return output;
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    return input.trim().to_string().to_lowercase();
}

fn list_vec(list: &Vec<String>, message: String) {
    println!("{}", message);
    for (index, element) in list.iter().enumerate() {
        println!("{} | {}", index, element);
    }   
}

fn select_from_vec(list: &Vec<String>) -> usize {
    let index: usize;
    loop {
        println!("Select an index: ");
        let input = read_input();
        match input.parse::<usize>() {
            Ok(value) => {
                index = value;
                break;
            }
            _ => println!("Invalid input."),
        }
    }
    if index < list.len() {
        return index;
    } else {
        println!("Invalid index. Please enter a valid index.");
        return select_from_vec(list);
    }
}

fn open_undead_config(filename: &String) -> io::Result<Undead> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut undead: Undead = serde_json::from_str(&contents)?;
    undead.hp = undead.roll_hp();
    println!("Success reading from file: {}.", filename);
    Ok(undead) 
}

fn get_undead_list(necromancer: &Necromancer) -> Vec<String> {
    necromancer.summons.iter().map(|summon| summon.name.clone()).collect::<Vec<String>>()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Necromancer {
    pub str:  i8,
    pub dex : i8,
    pub con: i8,
    pub int: i8,
    pub wis:  i8,
    pub cha: i8,
    pub summons : Vec<Undead>,
    pub level: u8,
    pub cr_thrall_max: u8, // multipiled by 4
    pub spell_bonus: i8,
}
impl Necromancer {
    pub fn attack(&self) {
        let mut undead_list = self.summons.clone();
        let mut index;
        while undead_list.len() > 0 {
            for (index, element) in undead_list.iter().enumerate() {
                println!("{} | {} | {}hp", index, element.name, element.hp);
            } 
            loop {
                println!("Select and undead to attack: ");
                let input = read_input();
                if input.is_empty() {
                    index = 0;
                    break;
                }
                match input.parse::<usize>() {
                    Ok(value) => {
                        index = value;
                        break;
                    }
                    _ => println!("Invalid input."),
                }
            } 
            println!("Do you have advantage");
            let has_advantage = user_input_yn();
            let mut has_disadvantage = false;
            if !has_advantage {
                println!("Do you have disadvantage");
                has_disadvantage = user_input_yn();
            }
            let advantage_attack = (has_disadvantage as u8) * 2 + (has_advantage as u8); // TODO: This Logic is Weird and Needs a rewrite
            println!("Hit:    {}", undead_hit(undead_list[index].hit, advantage_attack));
            println!("Damage: {}", undead_damage(undead_list[index].damage));
            undead_list.remove(index);
        }
        
    }

    fn add_summon_cr(&self) -> u8 {
        self.summons.iter().map(|s| s.cr).sum() 
    }

    pub fn summon(&mut self,mut path: PathBuf) {
        path = PathBuf::from(format!("{}/Undead", path.display()));
        let undead_list: Vec<String> = get_summonable_undead(&path).unwrap(); // unwrap cannot fail
        list_vec(&undead_list, "Select undead from list".to_string());
        let index = select_from_vec(&undead_list);
        let undead_path = format!("{}/{}", path.display(), &undead_list[index]);
        let mut undead = open_undead_config(&undead_path).unwrap(); // unwrap cannot fail unless the file has changed 
        let cr_total = self.add_summon_cr();
        if self.cr_thrall_max >= cr_total + undead.cr {
            if undead.name == "Skeleton" {
                println!("Does this skeleton have a bow");
                if user_input_yn(){
                    undead.name = "Skeleton with Bow".to_string(); 
                } else {
                    undead.name = "Skeleton with Sword".to_string();
                }
            }
            self.summons.push(undead);
        } else {
            println!("You cannot summon any more thralls");
            println!("Current total: {}/{}", cr_total, self.cr_thrall_max);
            println!("Creature CR  : {}", undead.cr);
        }
    }

    pub fn list(&self)  {
        println!("{}/{}", self.add_summon_cr(), self.cr_thrall_max);
        for (index, element) in self.summons.iter().enumerate() {
            println!("{} | {} | {}hp | ac:{}", index, element.name, element.hp, element.ac);
        }
    }

    pub fn change_health(&mut self){ // TODO: BREAK THIS FUNCTION DOWN
        let descriptions = get_undead_list(&self); 
        list_vec(&descriptions, "Select Undead that has lost/healed health".to_string());
        let index = select_from_vec(&descriptions);
        let change_in_health;
        loop {
            println!("How much health did the undead lose/heal: ");
            let input = read_input();
            match input.parse::<i8>() {
                Ok(value) => {
                    change_in_health = value;
                    break;
                }
                _ => println!("Invalid input."),
            }
        }

        self.summons[index].hp += &change_in_health; 
        
        if self.summons[index].hp > 0 {
            return // Early return to avoid Undead Fortitude logic
        } 

        if self.summons[index].name != "Zombie" { // Undead Fortitude
            println!("The {} has died", self.summons[index].name);
            self.summons.remove(index); // If the creature is not a zombie then remove it
        } else {
            if undead_fortitude(-change_in_health){
                self.summons.remove(index);
            } else {
                self.summons[index].hp = 1;
            }
        }
    }
}

fn undead_fortitude(damage: i8) -> bool {
    println!("Was the damage radiant or a crit");
    let is_radiant_or_crit: bool = user_input_yn();
    if  is_radiant_or_crit {
        println!("The zombie died"); 
        return true; 
    }
    if rand::thread_rng().gen_range(1..=20) >= damage + 5  {
        println!("The zombie survived with 1hp"); 
        return false;
    } else {
        println!("The zombie has died");
        return true;
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Undead {
    pub hp: i8,
    pub ac: u8,
    pub cr: u8,
    pub name: String,
    pub hit: i8,
    pub damage: (u8, u8, i8), // [Number of dice, Value of dice, modifier]
    hit_dice: (u8, u8, i8),
}

impl Undead {
    pub fn roll_hp(&self) -> i8 {
        let mut rng = rand::thread_rng();
        let number_of_dice = self.hit_dice.0;
        let value_of_dice = self.hit_dice.1;
        let modifier = self.hit_dice.2;
        let mut total: i8 = 0; 
        for _ in 0..number_of_dice{
            total += rng.gen_range(1..=value_of_dice) as i8;
        }
        return total + modifier

    }
}
