#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

// --- rocket stuff
#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
// --- rocket stuff

use rand::prelude::*;
use rand_derive2::RandGen;
use tracing::info;

// --- my stuff ---
mod traits;
use crate::traits::List::*;

mod implementations;
use crate::implementations::List::*;

mod functions;
use functions::*;

mod enums;
use crate::enums::List::*;

mod launch_pad;
use crate::launch_pad::*;

mod structs;
use crate::structs::List::*;

mod dice_bag;
use dice_bag::*;



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
