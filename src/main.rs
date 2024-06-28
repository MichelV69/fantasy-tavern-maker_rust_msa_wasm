#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use is_vowel::*;
use rand::prelude::*;
use rand_derive2::RandGen;

use inflector::string::singularize::to_singular;
use std::fmt;
use tracing::info;

// ---
mod dice_bag;
mod enums;
mod functions;
mod structs;

use crate::enums::List::*;
use crate::structs::List::*;
use dice_bag::*;
use functions::*;

// ---
trait StatSheet {
    fn stat_data() -> String;
}

impl PBHouse {
    fn new() -> Self {
        let eql = get_establishment_quality();
        PBHouse {
            name: get_name(),
            mood: get_mood(),
            lighting: get_lighting(),
            smells: get_smells(),
            size: get_pb_house_size(),
            establishment_quality: eql.clone(),
            posted_sign: get_posted_sign(),
            house_drink: get_house_drink(eql.level),
            house_dish: get_house_dish(eql.level),
        }
    }
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
            "\n-----                        Player Blurb                        -----".to_string(),
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

        pb_house_desc.push("\n \n ".to_string());
        pb_house_desc.push("The menu has the usual standard fare posted.".to_string());

        pb_house_desc.push(format!(
            "The House specialty beverage is {}, for {},",
            self.house_drink.desc, self.house_drink.price
        ));

        pb_house_desc.push(format!(
            " while the House specialty dish is {}, for {},",
            self.house_dish.desc, self.house_dish.price
        ));

        /*
             The House Specialty Drink is the
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
fn main() {
    info!("--- start of output ---");

    let pub_and_bed_house = PBHouse::new();
    println!("|| {} |", pub_and_bed_house);

    info!("--- end of output ---");
}
// --- eof ---
