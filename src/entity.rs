use std::{collections::HashMap, io};

use strum::IntoEnumIterator;

use self::profs::*;

/*
    put abilities in a list or something for the entity
*/
pub struct Entity {
    pub name: String,
    // I don't know how to abstract all the enums together to put them all in a array / generalize them
    // so for now there is this somewhat clunky situation
    pub abilities: (Ability<profs::StrengthProfs>,
        Ability<profs::DexterityProfs>,
        Ability<profs::ConstitutionProfs>,
        Ability<profs::IntelligenceProfs>,
        Ability<profs::WisdomProfs>,
        Ability<profs::CharismaProfs>
    ),
}

impl Entity {
    // TODO: distill functions, work on Ability's first
    pub fn new_generic(name: String) -> Self {

        // find solution with less repeated lines
        let str = Ability::<StrengthProfs>::new_generic();
        let dex = Ability::<DexterityProfs>::new_generic();
        let con = Ability::<ConstitutionProfs>::new_generic();
        let int = Ability::<IntelligenceProfs>::new_generic();
        let wis = Ability::<WisdomProfs>::new_generic();
        let cha = Ability::<CharismaProfs>::new_generic();

        Self {
            name,
            abilities: (str, dex, con, int, wis, cha),
        }
    }

    pub fn new_cli_ask(name: String) -> Self {
        // find solution with less repeated lines
        let str = Ability::<StrengthProfs>::new_cli_ask();
        let dex = Ability::<DexterityProfs>::new_cli_ask();
        let con = Ability::<ConstitutionProfs>::new_cli_ask();
        let int = Ability::<IntelligenceProfs>::new_cli_ask();
        let wis = Ability::<WisdomProfs>::new_cli_ask();
        let cha = Ability::<CharismaProfs>::new_cli_ask();

        Self {
            name,
            abilities: (str, dex, con, int, wis, cha),
        }
    }
}

/*
    add abilities and their specific proficiencies here
    note: ability level will be added later on in the Ability struct
    also: the boolean value will be mapped on later in a hashmap
*/
pub mod profs {

    // could all these be in a mod to make generics easier? (shorter)
    // at the very least it allows me to fold them all
    use strum_macros::{EnumIter, Display};
    #[derive(EnumIter, Debug, Hash, PartialEq, Eq, Display)]
    pub enum StrengthProfs {
        SavingThrows,
        Athletics,
    }
    #[derive(EnumIter, Debug, Hash, PartialEq, Eq, Display)]
    pub enum DexterityProfs {
        SavingThrows,
        Acrobatics,
        SleightOfHand,
        Stealth,
    }
    #[derive(EnumIter, Debug, Hash, PartialEq, Eq, Display)]
    pub enum ConstitutionProfs {
        SavingThrows,
    }
    #[derive(EnumIter, Debug, Hash, PartialEq, Eq, Display)]
    pub enum IntelligenceProfs {
        SavingThrows,
        Arcana,
        History,
        Investigation,
        Nature,
        Religion,
    }
    #[derive(EnumIter, Debug, Hash, PartialEq, Eq, Display)]
    pub enum WisdomProfs {
        SavingThrows,
        AnimalHandling,
        Insight,
        Medicine,
        Perception,
        Survival,
    }
    #[derive(EnumIter, Debug, Hash, PartialEq, Eq, Display)]
    pub enum CharismaProfs {
        SavingThrows,
        Deception,
        Intimidation,
        Performance,
        Persuasion,
    }
}

/*
        --😎-- abstract logic --😎--
*/

/*
    T becomes the enum for the stat you are creating
*/
pub struct Ability<T> {
    // make sure pub
    pub level: i8,
    pub modifier: i8,
    pub profs: HashMap<T, bool>,
}

impl<T: IntoEnumIterator + std::cmp::Eq + std::hash::Hash + std::fmt::Display> Ability<T> {

    // TODO: distill all down to one core function with layers on top
    pub fn new_generic() -> Self {

        // map the enum's values to boolean values
        let mut proficiencies = HashMap::new();
        let level = 10;

        for entry in T::iter() {
            // ofc some of these will be true in the final product, but those are selected by the user
            // so just a line or two getting the user's input for each would be needed
            proficiencies.insert(entry, false);
        }
        // build struct
        Self {
            level,
            modifier: get_modifier(level),
            profs: proficiencies,
        }
    }

    pub fn new_cli_ask() -> Self {

        // TODO: make one-liner functions for getting input
        let mut input = String::new();
        println!("Enter your level for {}: ", T.to_string()); // :shrug:
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let level = input.trim().parse::<i8>().unwrap();
        
        
        
        // map the enum's values to boolean values
        let mut proficiencies = HashMap::new();
        
        for entry in T::iter() {
            // ofc some of these will be true in the final product, but those are selected by the user
            // so just a line or two getting the user's input for each would be needed
            println!("Proficient in: {}?", entry);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
        
            // convert input to number
            let parsed = input.trim().parse::<i8>().unwrap();
            // convert to bool and put in map
            let val = if parsed == 0 {false} else {true};

            proficiencies.insert(entry, val);
        }

        
        // build struct
        Self {
            level,
            modifier: get_modifier(level),
            profs: proficiencies,
        }
    }

}

// just a match statement
fn get_modifier(level: i8) -> i8 {
    /*
        I would like this to be in the impl for Ability but you cant call it in the functions there
        /or/ you can, idk, but that's too much work
    */
    let modifier = match level {
        // lowest possible i8 number to 1
        i8::MIN..=1 => -5,
        2 | 3 => -4,
        4 | 5 => -3,
        6 | 7 => -2,
        8 | 9 => -1,
        10|11 => 0,
        12|13 => 1,
        14|15 => 2,
        16|17 => 3,
        18|19 => 4,
        20|21 => 5,
        22|23 => 6,
        24|25 => 7,
        26|27 => 8,
        28|29 => 9,
        30..=i8::MAX => 10,
        // 30 to hightest possible i8 number
    };
    modifier as i8
}