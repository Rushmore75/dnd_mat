use strum_macros::{EnumIter, Display};
use std::{collections::HashMap, fmt::Display, hash::Hash};
use strum::IntoEnumIterator;
use crate::input_and_output::get_input_parsed_wmsg;

/*
        --😎-- abstract logic --😎--

T becomes the enum for the stat you are creating

*/
pub struct Proficiency<T> {
    // make sure pub-lic
    pub name: String,
    pub level: i8,
    pub modifier: i8,
    pub profs: HashMap<T, bool>,
}

impl<T: IntoEnumIterator + Eq + Hash + Display> Proficiency<T> {

    pub fn new_cli_ask(category_name: &str) -> Self {
        // the cli asking should be done in entity not here

        // I need "T.name()" basically, but idk how to get that info
        let level = get_input_parsed_wmsg::<i8>(format!("Enter your level for {}", category_name)).unwrap();

        // map the enum's values to boolean values
        let mut proficiencies = HashMap::new();
        
        for entry in T::iter() {

            // get command-line input and parse into a (signed) 8 bit number
            let parsed = get_input_parsed_wmsg::<i8>(format!("Proficient in: {}?", entry)).unwrap();

            // convert number to bool, I think parsing a bool would be a little harder than just
            // doing this. IDK tho, haven't tried
            let val = if parsed == 0 {false} else {true}; // there isn't a turnary operator :(
            
            // enter value in map with the currently selected attribute
            proficiencies.insert(entry, val);
        }

        // build function
        Proficiency::new(category_name, level, proficiencies)
    }

    pub fn new_ext_hashmap(category_name: &str, level: i8, proficiencies: HashMap<T, bool>) -> Self {
        /*
            This is probably a bad idea to expose the hashmap creation but it 
            might come in useful when scripting entities
        */
        Proficiency::new(category_name, level, proficiencies)
    }

    fn new(category_name: &str, level: i8, proficiencies: HashMap<T, bool>) -> Self {
        /*
            The head honcho for creating the actual struct
        */
        Self {
            name: category_name.to_string(),
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

// fn get_name<T>() -> String {
    
//     match T {
//         StrengthProfs::Athletics => "Thing".to_string(), 
//     }
// }

/* 
Could all these be in a mod to make generics easier? (shorter)

The #[derive(xyz)] are there basically as bounds for generics so
methods don't freak out. They wouldn't be needed if it wasn't
abstracted like this as methods would implicitly see that they are
good to go. 

"Nothing really stops me from making these just lists of strings"
This removes "magic values" from being needed. You can later pass
an enum value to the hashmap to retrieve values instead of strings.
Makes cleaner code in practice.
*/

/*
    add abilities and their specific proficiencies here
    note: ability level will be added later on in the Ability struct
    also: the boolean value will be mapped on later in a hashmap
*/
/*
    Can i just make my own "enum" that actually fits my needs?
    - iterator
    - access to the name
*/

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
