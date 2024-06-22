#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use strum_macros::Display;
use strum_macros::EnumString;

use is_vowel::*;
use rand::prelude::*;
use rand_derive2::RandGen;

use std::fmt;

fn main() {
    println!("--- start of code ---");

    pub fn trim_whitespace(s: String) -> String {
        let words: Vec<_> = s.split_whitespace().collect();
        words.join(" ")
    }

    pub fn enum_to_text(s: String) -> String {
        let mut result = "".to_string();
        for c in s.chars() {
            result = if c.to_string() == c.to_lowercase().to_string() {
                format!("{}{}", result, c)
            } else {
                format!("{} {}", result, c.to_lowercase().to_string())
            };
        }
        result
    }

    // ---
    #[derive(Debug)]
    struct PBHouse {
        name: String,
        mood: String,
        lighting: String,
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

        format!("'{} {}'", verb.to_string(), noun.to_string())
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
                lighting: get_lighting(),
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
        let result = current_mood.to_string();
        trim_whitespace(enum_to_text(result))
    }
    // ---

    trait StatSheet {
        fn stat_data() -> String;
    }

    impl fmt::Display for PBHouse {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for line in &self.stat_data() {
                write!(f, "{}", line)?;
            }
            Ok(())
        }
    }

    impl PBHouse {
        fn stat_data(&self) -> Vec<String> {
            let mut pb_house_desc: Vec<String> = Vec::new();
            pb_house_desc.push(format!("The {}", self.name));
            // ---
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
            pb_house_desc.push(format!(" has a reputation for {prep} {} mood.", self.mood));
            // ---
            pb_house_desc.push(format!(" You can see that {} ", self.lighting));
            // ---
            return pb_house_desc;
        }
    }
    // ----
    #[derive(Display, RandGen)]
    enum LightingAdjectives {
        Brightly,
        Clearly,
        Evenly,
        Dimly,
        Shadowly,
    }
    #[derive(Display, RandGen)]
    enum LightingVerb {
        Lit,
        Illuminated,
    }
    #[derive(Display, RandGen)]
    enum LightingSources {
        Candles,
        AFireplace,
        OilLamps,
        MagicOrbsAndCrystals,
    }

    fn get_lighting() -> String {
        let adjective: LightingAdjectives = rand::random();
        let verb: LightingVerb = rand::random();
        let source: LightingSources = rand::random();
        let result = format!(" The main area is {} {} by {}.", adjective, verb, source);
        trim_whitespace(enum_to_text(result))
    }

    // ---
    #[derive(Debug)]
    struct EstablishmentQuality {
        quality: String,
        rooms: String,
        meals: String,
    }

    // --- main code ---
    println!("--- start of output ---");

    let pub_and_bed_house = PBHouse::new();
    println!("|| {} |", pub_and_bed_house);

    println!("--- end of output ---");
    // --- eof ---
}
