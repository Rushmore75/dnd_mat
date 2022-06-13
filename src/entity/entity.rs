use super::stats::proficiencies::*;

pub struct Entity {
    pub name: String,
    // I don't know how to abstract all the enums together to put them all in a array / generalize them
    // so for now there is this somewhat clunky situation
    pub abilities: (
        Proficiency<StrengthProfs>,
        Proficiency<DexterityProfs>,
        Proficiency<ConstitutionProfs>,
        Proficiency<IntelligenceProfs>,
        Proficiency<WisdomProfs>,
        Proficiency<CharismaProfs>
    ),
}

impl Entity {
    /*
    Entry point into creating an entity
    */

    // TODO: distill functions, work on Ability's distillation first
    // pub fn new_generic(name: String) -> Self {

    //     // find solution with less repeated lines
    //     let str = Proficiency::<StrengthProfs>::new_generic();
    //     let dex = Proficiency::<DexterityProfs>::new_generic();
    //     let con = Proficiency::<ConstitutionProfs>::new_generic();
    //     let int = Proficiency::<IntelligenceProfs>::new_generic();
    //     let wis = Proficiency::<WisdomProfs>::new_generic();
    //     let cha = Proficiency::<CharismaProfs>::new_generic();

    //     Self {
    //         name,
    //         abilities: (str, dex, con, int, wis, cha),
    //     }
    // }

    pub fn new_cli_ask(name: String) -> Self {
        // find solution with less repeated lines
        let str = Proficiency::<StrengthProfs>::new_cli_ask("Strength");
        let dex = Proficiency::<DexterityProfs>::new_cli_ask("Dexterity");
        let con = Proficiency::<ConstitutionProfs>::new_cli_ask("Constitution");
        let int = Proficiency::<IntelligenceProfs>::new_cli_ask("Intelligence");
        let wis = Proficiency::<WisdomProfs>::new_cli_ask("Wisdom");
        let cha = Proficiency::<CharismaProfs>::new_cli_ask("Charisma");

        Self {
            name,
            abilities: (str, dex, con, int, wis, cha),
        }
    }
}