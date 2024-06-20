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

// extern crate inquire;

use strum_macros::Display;
use strum_macros::EnumString;
use std::str::FromStr;
use inquire::Text;
use rand_derive2::RandGen;

fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    struct EstablishmentQuality
    {
        quality : String,
        rooms  : String,
        meals : String,
    }

    #[derive(Debug)]
    struct PBHouse {
        name: (NameVerb, NameNoun),
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
        cost_of_goods_index:String
    }

    #[derive(Debug, RandGen, Display)]
    enum NameVerb{
        Waltzing,Checkered,Lazy,Silver,Saucy,Flirting
    }
    #[derive(Debug, RandGen, Display)]
    enum NameNoun {
        Werebear,Cockrel,Hen,Dragon,Wench,Dryad
    }

    impl PBHouse {
    fn new() -> Self {
        PBHouse {
            name : (random(), random()),

        };

        fn get_name(self) -> String {
        format!("the {{self.name.0}}{{self.name.1}}")
        };
    }

    let mut pub_and_bed_house = PBHouse::new();
    pub_and_bed_house.get_name;
}
}
