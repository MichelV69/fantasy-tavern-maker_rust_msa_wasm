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
    // create new Tombstone
    assert!(Some(Tombstone::new(),true));

    // assign char_type
    // randomize gender
    // randomize partner_preference
    // randomize public_name
    // randomize task_desc
    // randomize race
} // mod tests

//--- end of file ---
