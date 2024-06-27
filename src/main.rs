#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use strum_macros::Display;
use strum_macros::EnumString;

use is_vowel::*;
use rand::prelude::*;
use rand_derive2::RandGen;

use inflector::string::singularize::to_singular;
use std::fmt;
use tracing::info;

mod fns;
use fns::*;

mod dicebag;
use dicebag::*;
use fns::ToCapitalized;

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
        posted_sign: String,
        // specialty_drink: String,
        // specialty_food: String,
        // establishment_history_notes: String,
        // redlight_services: String,
        establishment_quality: EstablishmentQuality,
        // cost_of_goods_index: String,
    }

    // ---
    // trait Name {
    //     fn name(&self) -> String;
    // }
    //
    // impl PBHouse {
    //     fn name(&self) -> String {
    //         "he {{name_verb}}{{name_noun}}"
    //             .to_string()
    //             .to_capitalized()
    //     }
    // }

    // ---
    impl PBHouse {
        fn new() -> Self {
            PBHouse {
                name: get_name(),
                mood: get_mood(),
                lighting: get_lighting(),
                smells: get_smells(),
                size: get_pb_house_size(),
                establishment_quality: get_establishment_quality(),
                posted_sign: get_posted_sign(),
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

            pb_house_desc.push(
                "\n-----                        Player Blurb                        -----"
                    .to_string(),
            );
            pb_house_desc.push("\n \n ".to_string());
            pb_house_desc.push(format!(
                "'The {}' is the local Pub and Bed House for travellers in this area.",
                self.name
            ));
            pb_house_desc.push(format!(
                " The {}-quality establishment would be considered {}, with {} tables.",
                trim_whitespace(enum_string_to_phase(
                    self.establishment_quality.level.to_string()
                )),
                trim_whitespace(enum_string_to_phase(self.size.size_description.to_string())),
                self.size.table_count
            ));

            let bed_type_name = if self.size.common_bed_count == 1 {
                to_singular(&tidy(self.size.common_bed_type.to_string()))
            } else {
                tidy(self.size.common_bed_type.to_string())
            };
            pb_house_desc.push(format!(
                " It has {} {} in the common room and {} private rooms.",
                self.size.common_bed_count, bed_type_name, self.size.private_room_count
            ));

            pb_house_desc.push(format!(
                " Rooms are {} per day, and meals are {} per day.",
                self.establishment_quality.rooms, self.establishment_quality.meals
            ));
            pb_house_desc.push("\n \n ".to_string());
            pb_house_desc.push(format!(
                " As you enter, the air is full of the scents of {}.",
                self.smells
            ));
            pb_house_desc.push(format!(
                " The current patrons seem to be {prep} {} bunch, {}.",
                self.mood, self.lighting
            ));
            pb_house_desc.push(self.posted_sign.clone()); // NB: I don't trust this
                                                          /*
                                                              The menu has the usual standard fare posted. The House Specialty Drink is the
                                                                  {House's own Hoppy, pale Ale}, for {16 copper},
                                                                  while the House Specialty Meal is {ground-pit charcoaled sausage,
                                                                  served with mushrooms}, for {16 copper}.
                                                          */
            // pb_house_desc.push(format!(" lore ipsum",xx);
            // ---
            // ---

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
        Hammocks,
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
        let our_pbhouse: PBHouseSize = match pb_size {
            SizeList::Tiny => {
                let pb_tables_roll = dicebag::tower::DiceResult::from_string("2d4");
                let pb_tables = pb_tables_roll.get_total();

                let pb_beds_roll = dicebag::tower::DiceResult::from_string("1d4");
                let pb_beds = pb_beds_roll.get_total();

                PBHouseSize {
                    size_description: pb_size,
                    table_count: pb_tables,
                    common_bed_type: BedTypeList::Hammocks,
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
    #[derive(Debug)]
    struct EstablishmentQuality {
        level: EstablishmentQualityLevel,
        rooms: String,
        meals: String,
    }

    #[derive(Debug, Display, RandGen)]
    enum EstablishmentQualityLevel {
        Squalid,
        Poor,
        Modest,
        Comfortable,
        Wealthy,
        Aristocratic,
    }

    fn get_establishment_quality() -> EstablishmentQuality {
        let establishment_quality_level: EstablishmentQualityLevel = random();
        let cost_data = match establishment_quality_level {
            EstablishmentQualityLevel::Squalid => ("7cp", "3cp"),
            EstablishmentQualityLevel::Poor => ("1sp", "6cp"),
            EstablishmentQualityLevel::Modest => ("5sp", "3sp"),
            EstablishmentQualityLevel::Comfortable => ("8sp", "5sp"),
            EstablishmentQualityLevel::Wealthy => ("2gp", "8sp"),
            EstablishmentQualityLevel::Aristocratic => ("4gp", "2gp"),
        };

        EstablishmentQuality {
            level: establishment_quality_level,
            rooms: cost_data.0.to_string(),
            meals: cost_data.1.to_string(),
        }
    }

    // --- main code ---
    info!("--- start of output ---");

    let pub_and_bed_house = PBHouse::new();
    println!("|| {} |", pub_and_bed_house);

    info!("--- end of output ---");
    // --- eof ---
}
