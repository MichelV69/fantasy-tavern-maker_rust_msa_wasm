#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

// --- rocket stuff
#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
// --- rocket stuff

use is_vowel::*;
use rand::prelude::*;
//use rand_derive2::RandGen;

use inflector::string::singularize::to_singular;
use std::fmt;
use tracing::info;

// ---
mod dice_bag;
mod enums;
mod functions;
mod launch_pad;
mod structs;

use crate::enums::List::*;
use crate::launch_pad::*;
use crate::structs::List::*;

use dice_bag::*;
use functions::*;

// ---
trait AppFn {
    fn get_version(&self) -> String;
}

impl AppFn for App {
    fn get_version(&self) -> String {
        format!(
            "{}.{}.{}",
            self.version_major, self.version_minor, self.version_fix
        )
    }
}

impl App {
    fn new() -> Self {
        App {
            name: "Fantasy Tavern Maker".into(),
            version_major: 0,
            version_minor: 6,
            version_fix: 0,
        }
    }
}

// ---
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
        write!(
            f,
            "{}",
            "\n-----                        Player Blurb                        -----"
        )?;
        for line in &self.general_info() {
            write!(f, "{}", line)?;
        }

        write!(
            f,
            "{}",
            "\n -----                          DM Notes                          -----"
        )?;
        for line in &self.history_profile() {
            write!(f, "{}", line)?;
        }

        for line in &self.redlight_profile() {
            write!(f, "{}", line)?;
        }

        for line in &self.staff_and_customers() {
            write!(f, "{}", line)?;
        }

        Ok(())
    }
}

// ---
impl PBHouse {
    fn general_info(&self) -> Vec<String> {
        let mut pb_house_desc: Vec<String> = Vec::with_capacity(22);
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

        let para1: String = format!(
            "'*The {}*' is the local Pub and Bed House for travellers in this area.
            The {}-quality establishment would be considered {}, with {} tables.",
            self.name,
            trim_whitespace(enum_string_to_phase(
                self.establishment_quality.level.to_string()
            )),
            trim_whitespace(enum_string_to_phase(self.size.size_description.to_string())),
            self.size.table_count
        );
        pb_house_desc.push(para1);

        let bed_type_name = if self.size.common_bed_count == 1 {
            to_singular(&tidy(self.size.common_bed_type.to_string()))
        } else {
            tidy(self.size.common_bed_type.to_string())
        };
        let para2: String = format!(
            "It has {} {} in the common room and {} private rooms.
            Rooms are *{}* per day, and meals are *{}* per day.",
            self.size.common_bed_count,
            bed_type_name,
            self.size.private_room_count,
            self.establishment_quality.rooms,
            self.establishment_quality.meals
        );
        pb_house_desc.push(para2);

        let para3: String = format!(
            "As you enter, the air is full of the scents of {}. The current patrons seem to be {prep} {} bunch, {}. {}",
            self.smells, self.mood, self.lighting, self.posted_sign.clone()
        );
        pb_house_desc.push(para3);

        let para4: String = format!(
            "The menu has the usual standard fare posted.
            The House specialty beverage is {}, for {},
            while the House specialty dish is {}, for {}.",
            self.house_drink.desc,
            self.house_drink.price,
            self.house_dish.desc,
            self.house_dish.price
        );
        pb_house_desc.push(para4);

        // ---
        pb_house_desc
    }

    fn history_profile(&self) -> Vec<String> {
        let mut pb_house_desc: Vec<String> = Vec::with_capacity(22);

        pb_house_desc.push(format!(
            "* The *{}* is {}. \n * {}. \n * {}.",
            self.name,
            get_establishment_history_age(),
            get_establishment_appearance(),
            get_establishment_reputation()
        ));

        // ---
        pb_house_desc
    }

    fn redlight_profile(&self) -> Vec<String> {
        let mut pb_house_desc: Vec<String> = Vec::with_capacity(22);
        let red_light_services_list = get_red_light_services_list();
        if red_light_services_list.is_some() {
            pb_house_desc.push(format!(
                "{}",
                red_light_services_list.expect("Should always be String.")
            ))
        }
        // ---
        pb_house_desc
    }

    fn staff_and_customers(&self) -> Vec<String> {
        let mut pb_house_desc: Vec<String> = Vec::with_capacity(22);
        /*
          -----                  Notable Staff & Patrons                  -----
        Staff : (Character) is the Owner. They are a male human; average height (3%) and
        stout (+13%). They are hazel-eyed, with their white hair kept in long curls.
        [GM Notes: They consider themselves hetro.  (Quirks:  They have a slight scar on
        their right shoulder.  They are often distrustful of adventurers. ) Particularly
        Good At: [(Int) Arcana: +2] Particularly Bad At: [(Wis) Animal Handling: -3]]

        pb_house_desc.push(format!(" lore ipsum: {}", "Muspi erol");
        */

        // ---
        pb_house_desc
    }
} // --- impl PBHouse

// --- web server code

#[launch]
fn rocket() -> _ {
    println!(
        ">>>  Booted Directory: [{}]",
        std::env::current_dir().expect("STD ENV info").display()
    );
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![version])
        .mount("/styles", FileServer::from("content/css"))
}

// --- local cli code
fn app() -> String {
    let pub_and_bed_house = PBHouse::new();
    format!("\n \n {} \n \n", pub_and_bed_house)
}
// --- eof ---
