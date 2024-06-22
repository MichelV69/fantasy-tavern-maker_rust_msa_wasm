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

mod fns;
use fns::*;

fn main() {
    println!("--- start of code ---");

    // ---
    #[derive(Debug)]
    struct PBHouse {
        name: String,
        mood: String,
        lighting: String,
        smells: String,
        // size: String,
        // posted_sign: String,
        // specialty_drink: String,
        // specialty_food: String,
        // establishment_history_notes: String,
        // redlight_services: String,
        // establishment_quality: EstablishmentQuality,
        // cost_of_goods_index: String,
    }

    // ---
    trait Name {
        fn name(&self) -> String;
    }

    impl PBHouse {
        fn name(&self) -> String {
            "the {{name_verb}}{{name_noun}}".to_string()
        }
    }

    // ---
    impl PBHouse {
        fn new() -> Self {
            PBHouse {
                name: get_name(),
                mood: get_mood(),
                lighting: get_lighting(),
                smells: get_smells(),
            }
        }
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

    // ---
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
            pb_house_desc.push(format!(" You can see that {} .", self.lighting));
            // ---
            pb_house_desc.push(format!(" The air here is full of the {}.", self.smells));
            // ---
            pb_house_desc
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
    println!("--- start of output ---");

    let pub_and_bed_house = PBHouse::new();
    println!("|| {} |", pub_and_bed_house);

    println!("--- end of output ---");
    // --- eof ---
}
