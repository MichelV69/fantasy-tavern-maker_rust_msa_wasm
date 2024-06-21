#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

//use mini_redis::{client, Result as redis_Result};
//use color_eyre::eyre::Result;
//use tracing::info;
//use std::collections::HashMap;
//use rayon::prelude::*;
//use chrono::prelude::{DateTime, Local};

// inquire crate, a terrific command line interaction crate that allows
//      us to build interactive menus, auto-completing lists, input
//      lines with good readline support and more.
// extern crate inquire;
// use inquire::Text;

use std::str::FromStr;
use strum_macros::Display;
use strum_macros::EnumString;

// use rand::random;
use rand::prelude::*;
use rand_derive2::RandGen;

fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    struct PBHouse {
        name_verb: NameVerb,
        name_noun: NameNoun,
        mood: String,
        lighting: String,
        smells: String,
        size: String,
        posted_sign: String,
        specialty_drink: String,
        specialty_food: String,
        establishment_history_notes: String,
        redlight_services: String,
        establishment_quality: EstablishmentQuality,
        cost_of_goods_index: String,
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
    }

    #[derive(Debug, RandGen, Display, EnumString, Eq, PartialEq)]
    enum NameNoun {
        Werebear,
        Cockrel,
        Hen,
        Dragon,
        Wench,
        Dryad,
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
            };
        }
    }
    //    let mut pub_and_bed_house = PBHouse::new();
    //    pub_and_bed_house.get_name;
    // --- eof ---
}
