#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::error::Error;
use std::fmt;
use std::slice::Iter;
use std::str::FromStr;
use strum_macros::Display;
use strum_macros::EnumString;

use rand::prelude::*;
use rand_derive2::RandGen;

fn main() {
    println!("--- start of code ---");

    #[derive(Debug)]
    struct PBHouse {
        name_verb: NameVerb,
        name_noun: NameNoun,
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

    #[derive(Debug)]
    struct EstablishmentQuality {
        quality: String,
        rooms: String,
        meals: String,
    }
    trait New {
        fn new() -> PBHouse;
    }

    impl PBHouse {
        fn new() -> Self {
            PBHouse {
                name_verb: random(),
                name_noun: random(),
                mood: get_mood(),
            }
        }
    }

    fn get_mood() -> String {
        "this is a Big Mood".to_string()
    }

    trait StatSheet {
        fn stat_data() -> String;
    }

    impl PBHouse {
        fn stat_data(&self) -> Vec<String>{
            let mut pb_house_desc: Vec<String> = Vec::new();
            pb_house_desc.push(format!("the {} {}", self.name_verb, self.name_noun));
            pb_house_desc.push(format!("has a reputation for a {} mood.", self.mood));
            return pb_house_desc
        }
    }
    // --- main code ---
    let mut pub_and_bed_house = PBHouse::new();
    println!("--- start of output ---");

    for line in pub_and_bed_house.stat_data() {
        println!("-- {:#?} --", line);
    }
    println!("--- end of output ---");
    // --- eof ---
}
