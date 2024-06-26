#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use strum_macros::Display;
use strum_macros::EnumString;

use is_vowel::*;
use rand::prelude::*;
use rand_derive2::RandGen;

use tracing::info;

use std::fmt;

mod fns;
use fns::*;

mod dicebag;
use dicebag::*;

fn main() {
    info!("--- start of code ---");

    // ---
    #[derive(Debug)]
    struct PBHouse {
        name: String,
        mood: String,
        lighting: String,
        smells: String,
        size: PBHouseSize,
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
                size: get_pb_house_size(),
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
            pb_house_desc.push(format!(" You can see that {}.", self.lighting));
            // ---
            pb_house_desc.push(format!(
                " The air here is full of the smells of {}.",
                self.smells
            ));
            // ---
            pb_house_desc.push(format!(
                " There are {} tables in the common room.",
                self.size.table_count
            ));
            // ---
            pb_house_desc
        }
    }

    // ---
    #[derive(Debug, Display, RandGen)]
    enum SizeList {
        Tiny,
        Small,
        Modest,
        Large,
        Massive,
    }

    #[derive(Debug, Display)]
    enum BedTypeList {
        Hammock,
        BunkBeds,
        SingleBeds,
        TentBeds,
    }

    #[derive(Debug)]
    struct PBHouseSize {
        size_description: SizeList,
        table_count: i8,
        common_bed_type: BedTypeList,
        common_bed_count: i8,
        private_room_count: i8,
    }

    fn get_pb_house_size() -> PBHouseSize {
        let pb_size: SizeList = random();
        // --- pb_tables_roll dice should be based on pb_size
        let mut our_pbhouse: PBHouseSize = match pb_size {
            SizeList::Tiny => {
                let pb_tables_roll = dicebag::tower::DiceResult::from_string("2d4");
                let pb_tables = pb_tables_roll.get_total();

                let pb_beds_roll = dicebag::tower::DiceResult::from_string("1d4");
                let pb_beds = pb_beds_roll.get_total();

                PBHouseSize {
                    size_description: pb_size,
                    table_count: pb_tables,
                    common_bed_type: BedTypeList::Hammock,
                    common_bed_count: pb_beds,
                    private_room_count: 0,
                }
            }
            SizeList::Small => {
                let pb_tables_roll = dicebag::tower::DiceResult::from_string("3d4");
                let pb_tables = pb_tables_roll.get_total();

                let pb_beds_roll = dicebag::tower::DiceResult::from_string("2d4");
                let pb_beds = pb_beds_roll.get_total();

                let pb_priv_room_roll = dicebag::tower::DiceResult::from_string("1d4");
                let pb_priv_rooms = pb_priv_room_roll.get_total();
                PBHouseSize {
                    size_description: pb_size,
                    table_count: pb_tables,
                    common_bed_type: BedTypeList::BunkBeds,
                    common_bed_count: pb_beds,
                    private_room_count: pb_priv_rooms,
                }
            }
            SizeList::Modest => {
                let pb_tables_roll = dicebag::tower::DiceResult::from_string("4d6");
                let pb_tables = pb_tables_roll.get_total();

                let pb_beds_roll = dicebag::tower::DiceResult::from_string("3d6");
                let pb_beds = pb_beds_roll.get_total();

                let pb_priv_room_roll = dicebag::tower::DiceResult::from_string("2d6");
                let pb_priv_rooms = pb_priv_room_roll.get_total();
                PBHouseSize {
                    size_description: pb_size,
                    table_count: pb_tables,
                    common_bed_type: BedTypeList::SingleBeds,
                    common_bed_count: pb_beds,
                    private_room_count: pb_priv_rooms,
                }
            }
            SizeList::Large => {
                let pb_tables_roll = dicebag::tower::DiceResult::from_string("5d6");
                let pb_tables = pb_tables_roll.get_total();

                let pb_beds_roll = dicebag::tower::DiceResult::from_string("4d6");
                let pb_beds = pb_beds_roll.get_total();

                let pb_priv_room_roll = dicebag::tower::DiceResult::from_string("3d6");
                let pb_priv_rooms = pb_priv_room_roll.get_total();
                PBHouseSize {
                    size_description: pb_size,
                    table_count: pb_tables,
                    common_bed_type: BedTypeList::TentBeds,
                    common_bed_count: pb_beds,
                    private_room_count: pb_priv_rooms,
                }
            }
            SizeList::Massive => {
                let pb_tables_roll = dicebag::tower::DiceResult::from_string("7d8");
                let pb_tables = pb_tables_roll.get_total();

                let pb_beds_roll = dicebag::tower::DiceResult::from_string("6d8");
                let pb_beds = pb_beds_roll.get_total();

                let pb_priv_room_roll = dicebag::tower::DiceResult::from_string("4d8");
                let pb_priv_rooms = pb_priv_room_roll.get_total();
                PBHouseSize {
                    size_description: pb_size,
                    table_count: pb_tables,
                    common_bed_type: BedTypeList::TentBeds,
                    common_bed_count: pb_beds,
                    private_room_count: pb_priv_rooms,
                }
            }
        };
        // ---

        our_pbhouse
    }

    // ---
    // #[derive(Debug)]
    // struct EstablishmentQuality {
    //     quality: String,
    //     rooms: String,
    //     meals: String,
    // }

    // --- main code ---
    info!("--- start of output ---");

    let pub_and_bed_house = PBHouse::new();
    println!("|| {} |", pub_and_bed_house);

    info!("--- end of output ---");
    // --- eof ---
}
