#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use strum_macros::Display;
use strum_macros::EnumString;

use is_vowel::*;
use rand::prelude::*;
use rand_derive2::RandGen;

fn main() {
    println!("--- start of code ---");

    // ---
    #[derive(Debug)]
    struct PBHouse {
        name: String,
        mood: String,
        // lighting: String,
        // smells: String,
        // size: String,
        // posted_sign: String,
        // specialty_drink: String,
        // specialty_food: String,
        // establishment_history_notes: String,
        // redlight_services: String,
        // establishment_quality: EstablishmentQuality,
        // cost_of_goods_index: String,
    }

    trait Name {
        fn name(&self) -> String;
    }

    impl PBHouse {
        fn name(&self) -> String {
            format!("the {{name_verb}}{{name_noun}}")
        }
    }

    // ---

    #[derive(Debug, RandGen, Display, EnumString, Eq, PartialEq)]
    enum NameVerb {
        Waltzing,
        Checkered,
        Lazy,
        Silver,
        Saucy,
        Flirting,
        Blue,
        Red,
        Green,
        Yellow,
        Fickle,
        Roaring,
        Carousing,
        Melting,
        Drifting,
        Spring,
        Winter,
        Summer,
        Autumn,
        Pouring,
        Heaving,
    }

    #[derive(Debug, RandGen, Display, EnumString, Eq, PartialEq)]
    enum NameNoun {
        Werebear,
        Cockrel,
        Hen,
        Dragon,
        Wench,
        Dryad,
        Sky,
        Tide,
        Meadow,
        Sun,
        Fortune,
        Waters,
        Bard,
        Curmudgeon,
        Crystal,
        Mongrel,
        Ice,
        Tempest,
        Snows,
        Draft,
        Harvest,
        Chalice,
        Waves,
    }

    fn get_name() -> String {
        let verb: NameVerb = rand::random();
        let noun: NameNoun = rand::random();

        let mut result = format!("{} {}", verb.to_string(), noun.to_string());
        result
    }

    // ---

    trait New {
        fn new() -> PBHouse;
    }

    impl PBHouse {
        fn new() -> Self {
            PBHouse {
                name: get_name(),
                mood: get_mood(),
            }
        }
    }

    // ---

    #[derive(Debug, RandGen, Display, EnumString, Eq, PartialEq)]
    enum MoodData {
        Jovial,
        Relaxing,
        Smoky,
        Erudite,
        Loud,
        Subdued,
        Rowdy,
        Seedy,
        Shady,
        Busy,
        LowerClass,
        MiddleClass,
        UpperClass,
        MerchantFriendly,
        Dour,
        Flirty,
    }

    fn get_mood() -> String {
        let current_mood: MoodData = rand::random();
        let words = current_mood.to_string();

        let mut result = "".to_string();
        for c in words.chars() {
            result = if c.to_string() == c.to_lowercase().to_string() {
                format!("{}{}", result, c)
            } else {
                format!("{} {}", result, c.to_lowercase().to_string())
            }
        }

        result
    }
    // ---

    trait StatSheet {
        fn stat_data() -> String;
    }

    impl PBHouse {
        fn stat_data(&self) -> Vec<String> {
            let mut pb_house_desc: Vec<String> = Vec::new();
            pb_house_desc.push(format!("the {}", self.name));
            let mut first_char = self
                .mood
                .to_string()
                .chars()
                .nth(0)
                .expect("This should be a single character");
            let prep = if first_char.is_romance_vowel() {
                "an"
            } else {
                "a"
            };

            pb_house_desc.push(format!("has a reputation for {prep} {} mood.", self.mood));
            return pb_house_desc;
        }
    }

    // ---
    #[derive(Debug)]
    struct EstablishmentQuality {
        quality: String,
        rooms: String,
        meals: String,
    }

    // --- main code ---
    let pub_and_bed_house = PBHouse::new();
    println!("--- start of output ---");

    for line in pub_and_bed_house.stat_data() {
        println!("-- {:#?} --", line);
    }
    println!("--- end of output ---");
    // --- eof ---
}
