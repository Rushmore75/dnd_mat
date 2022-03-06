use strum_macros::{EnumIter, Display};
use std::{collections::HashMap, fmt::Display, hash::Hash};
use strum::IntoEnumIterator;
use crate::input_and_output::get_input;

/*
        --😎-- abstract logic --😎--

T becomes the enum for the stat you are creating

*/
pub struct Proficiency<T> {
    // make sure pub-lic
    pub level: i8,
    pub modifier: i8,
    pub profs: HashMap<T, bool>,
}

impl<T: IntoEnumIterator + Eq + Hash + Display> Proficiency<T> {

    // TODO: distill all the function's variants down to one core function with layers on top...
    // no duplicate code!

    pub fn new_cli_ask() -> Self {

        // I need "T.name()" basically, but idk how to get that info
        let level = 6;
        // let level = get_input(format!("Enter your level for {}", T)).parse::<i8>().unwrap();

        // map the enum's values to boolean values
        let mut proficiencies = HashMap::new();
        
        for entry in T::iter() {

            // get command-line input and parse into a (signed) 8 bit number
            let mut s = String::new(); 
            get_input(format!("Proficient in: {}?", entry), &mut s);
            let parsed = s.trim().parse::<i8>().unwrap();
            // ^^ yes this is weird, but "known memory size at compile" & "lifetime modifiers" kinda forced my hand
            
            // convert number to bool, I think parsing a bool would be a little harder than just
            // doing this. IDK tho, haven't tried
            let val = if parsed == 0 {false} else {true}; // there isn't a turnary operator :(
            
                // enter value in map with the currently selected attribute
            proficiencies.insert(entry, val);
        }

        // build struct
        Self {
            level,
            modifier: get_modifier(level), // checks against a predefined list for mods
            profs: proficiencies,
        }
    }

    
    pub fn new_generic() -> Self {
        // same as new_cli_ask(), but just set all the proficiencies to false.
        // possible easier to understand tho, less lines.

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
