use std::path::PathBuf;
use std::io::ErrorKind;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
mod init;
use init::Necromancer as Player;
use init::Undead as Summon;
use std::io::BufWriter;
use colored::Colorize;
use termion::clear;

fn main() {
    let mut necromancer = init_necromancer(get_save_file());
    loop {
        println!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "attack".yellow(),
            " - Automatically rolls for each undead\n",
            "health".yellow(),
            " - Lets you set the health of a undead\n",
            "cast".yellow(),
            " - casts animate dead\n",
            "level".yellow(),
            " - Gets player stats\n",
            "list".yellow(),
            " - lists undead\n",
            "update".yellow(),
            " - Change player stats\n",
            "save".yellow(),
            " - saves the current state\n",
            "clear".yellow(),
            " - clears the terminal\n"
        );

        let input = read_input();

        match input.as_str() {
            "attack" => necromancer.attack(),
            "health" => necromancer.change_health(),
            "cast" => necromancer.summon(get_config_dir()),
            "level" => get_player_stats(&necromancer),
            "list" => necromancer.list(),
            "update" => necromancer = create_necromancer(necromancer.summons),
            "save" => save(&necromancer),
            "clear" => println!("{}", clear::All), 
            _ => println!("Error: Unknown command"),
        }
    }
}

fn init_necromancer(filename: PathBuf) -> Player {
    match File::open(&filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                println!("Error reading file: {}. Creating new Necromancer.", e);
                return create_necromancer(Vec::new());
            }
            match serde_json::from_str(&contents) {
                Ok(necromancer) => {
                    println!("Success reading from file: {}.", filename.to_str().unwrap()); // Unwrap can never fail
                    necromancer
                },
                Err(e) => {
                    println!("Error parsing save file: {}. Creating new Necromancer.", e);
                    create_necromancer(Vec::new())
                }
            }
        }
        Err(_) => {
            println!("No save file found. Creating new Necromancer.");
            create_necromancer(Vec::new())
        }
    }
}
fn config_init() {
    let undead_config = PathBuf::from(format!("{}/Undead", get_config_dir().display()));
    let dir_path = [
        get_config_dir(),
        undead_config    
    ];

    for index in 0..2 { 
        match std::fs::metadata(&dir_path[index]) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    println!("Directory {} already exists.", dir_path[index].display());
                } else {
                    println!("{} is not a directory.", dir_path[index].display());
                }
            }
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    fs::create_dir_all(&dir_path[index]).expect("Failed to create directory");
                    println!("Directory {} created successfully.", dir_path[index].display());
                } else {
                    eprintln!("Error creating directory: {}", err);
                }
            }
        }
    }
}

fn save(necro: &Player) {
    config_init();
    let file = File::create(get_save_file()).unwrap(); // Unwrap can only fail when dir does not exist
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &necro).unwrap();
    println!("Save successful"); 
}

fn get_config_dir() -> PathBuf {
    let path = dirs::config_dir().expect("You must have a configuration file for your system").join("necrorust");
    if !path.exists() {
        config_init();
    }
    return path
} 

fn get_save_file() -> PathBuf {
    let mut path = get_config_dir();
    path.push("necrorust_save.json");
    return path
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    return input.trim().to_string().to_lowercase();
}

fn get_skills() -> [i8; 6] { // Actually good code writen by AI
    let attributes = [
        "Strength",
        "Dexterity",
        "Constitution",
        "Intelligence",
        "Wisdom",
        "Charisma",
    ];
    let mut stats = [0; 6];
    for (i, attr) in attributes.iter().enumerate() {
        loop {
            println!("Enter {} (-5-5): ", attr);
            let input = read_input();
            match input.parse::<i8>() {
                Ok(value) if (-5..=5).contains(&value) => {
                    stats[i] = value;
                    break;
                }
                _ => println!("Invalid input. Please enter a number between -5 and 5."),
            }
        }
    }
    return stats;
}
fn get_etc() -> [u8; 3] {
    println!("Enter HP: ");
    let hp = loop {
        match read_input().parse::<u8>() {
            Ok(value) => break value,
            _ => println!("Invalid input. Please enter a number between 1 and 20."),
        }
    };
    println!("Enter AC: ");
    let ac = loop {
                 match read_input().parse::<u8>() {
                     Ok(value) => break value,
                     _ => println!("Invalid input. Please enter a number between 1 and 20."),
                 }
    };

    println!("Enter Level (1-20)");
    let level = loop {
                 match read_input().parse::<u8>() {
                     Ok(value) if (1..=20).contains(&value) => break value,
                     _ => println!("Invalid input. Please enter a number between 1 and 20."),
                 }
    };
    return [hp, ac, level];
}

fn calc_cr(level: u8) -> u8{
    match level {
        1 => return 0,
        2 => return 1,
        3..=4 => return 2,
        5..=8 => return 4,
        9..=12 => return 8,
        13..=16 => return 12,
        17..=20 => return 16,
        _ => panic!("Your level is invalid",)
    }
}

fn create_necromancer(undead: Vec<Summon>) -> Player {
    println!("Let's create a new Necromancer!");
    let hpaclvl = get_etc();
    let stats = get_skills();
    let cr_max = calc_cr(hpaclvl[2]);
    let new_necromancer = Player {
        str: stats[0],
        dex: stats[1],
        con: stats[2],
        int: stats[3],
        wis: stats[4],
        cha: stats[5],
        summons: undead,
        level: hpaclvl[2],
        cr_thrall_max: cr_max,
        spell_bonus: ((hpaclvl[2] as f64 / 4.0).ceil() as i8 + 1) + stats[3],
    };
    save(&new_necromancer);
    return new_necromancer
}

fn get_player_stats(necromancer: &Player) {
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",
        "str: ".red(),
        necromancer.str,
        "\ndex: ".red(),
        necromancer.dex,
        "\ncon: ".red(),
        necromancer.con,
        "\nint: ".red(),
        necromancer.int,
        "\nwis: ".red(),
        necromancer.wis,
        "\ncha: ".red(),
        necromancer.cha, 
        "\nlvl: ".red(),
        necromancer.level,
        "\nspl: ".red(),
        necromancer.spell_bonus,
        );
}

