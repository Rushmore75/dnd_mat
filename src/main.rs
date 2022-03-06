use crate::entity::{entity::Entity, stats::proficiencies::*};


mod entity;
mod input_and_output;

fn main() {

    let pc = Entity::new_cli_ask("Steve".to_string());

    // testing 

    // these numbers could be exchanged for a `enum => number` match thingy?
    // it is referring to array indexes so idk
    println!("Proficient in str athletics? {:?}",        pc.abilities.0.profs.get(&StrengthProfs::Athletics).unwrap());
    println!("Proficient in dex ST? {:?}",               pc.abilities.1.profs.get(&DexterityProfs::SavingThrows).unwrap());
    println!("Proficient in wis animal handling? {:?}",  pc.abilities.4.profs.get(&WisdomProfs::AnimalHandling).unwrap());
        
    println!("{}, with level: {}, you get modifier: {}", pc.name, pc.abilities.1.level, pc.abilities.1.modifier);
}
