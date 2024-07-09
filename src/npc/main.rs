//--- start of file ---
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::launch;
use rocket::routes;
use strum::Display;
use strum::EnumIter;
use variant_count::VariantCount;
use rand_derive2::RandGen;

// --- my stuff ---
mod launch_pad;
use crate::launch_pad::*;
mod enums;
use crate::enums::List::*;
mod structs;
use crate::structs::List::*;
mod traits;
use crate::traits::List::*;
mod implementations;
use crate::implementations::List::*;

#[launch]
fn rocket() -> _ {
    println!(
        ">>>  Booted Directory: [{}]",
        std::env::current_dir().expect("STD ENV info").display()
    );
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8002)))
        .mount("/", routes![index])
        //        .mount("/", routes![version])
        .mount("/styles", FileServer::from("content/css"))
}

// --- tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::debug_assert;
    use tracing::{event, Level};

    // --- my stuff ---
    use crate::launch_pad::*;
    use crate::enums::List::*;
    use crate::structs::List::*;
    use crate::implementations::List::*;

    #[test]
    fn create_new_tombstone() {
        let test = Tombstone::new();
        assert_eq!(test.char_type, TypeCodeList::Drifter );
        assert_eq!(test.gender, GenderCodeList::Androgenous );
        assert_eq!(test.partner_preference, PartnerPreferenceCodeList::Hetro );
        assert_eq!(test.public_name, "Jane Q Publique" );
        assert_eq!(test.task_desc, "Wandering Wanderer", );
        assert_eq!(test.race, RaceCodeList::Human, );
    }

    #[test]
    fn assign_char_type(){
        let mut test = Tombstone::new();
        let char_type = TypeCodeList::Staff;
        test.char_type = char_type;
        assert_eq!(test.char_type, char_type );
    }

    #[test]
    fn randomize_race(){
         let mut test = Tombstone::new();
         let new_race = RaceCodeList::weighted_random();
         test.race = new_race;
         assert_eq!(test.race, new_race);
    }

    #[test]
    fn randomize_gender(){
        let mut test = Tombstone::new();
        let new_gender = GenderCodeList::weighted_random();
        test.gender = new_gender;
        assert_eq!(test.gender, new_gender);
    }

    // randomize partner_preference
    // randomize public_name
    // randomize task_desc
    //
} // mod tests

//--- end of file ---
