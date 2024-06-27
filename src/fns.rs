#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use is_vowel::*;
use rand::prelude::*;
use rand_derive2::RandGen;
use std::fmt;
use strum_macros::Display;
use strum_macros::EnumString;

use crate::DiceBag::Tower::*;
use crate::Enums::List::*;
use crate::Structs::List::*;

pub fn trim_whitespace(s: String) -> String {
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

pub fn enum_string_to_phase(s: String) -> String {
    let mut result = "".to_string();
    for c in s.chars() {
        result = if c.to_string() == c.to_lowercase().to_string() {
            format!("{}{}", result, c)
        } else {
            format!("{} {}", result, c.to_lowercase())
        };
    }
    result
}

pub fn tidy(s: String) -> String {
    trim_whitespace(enum_string_to_phase(s))
}

// from https://stackoverflow.com/questions/38342805/how-do-i-collect-from-multiple-iterator-types#
pub trait ToCapitalized {
    fn to_capitalized(&self) -> String;
}
impl ToCapitalized for str {
    fn to_capitalized(&self) -> String {
        match self.len() {
            0 => String::new(),
            _ => {
                let mut s = String::with_capacity(self.len());
                s.extend(self.chars().next().unwrap().to_uppercase());
                s.extend(self.chars().skip(1).flat_map(|c| c.to_lowercase()));
                s
            }
        }
    }
}

// ---
pub fn get_name() -> String {
    let verb: NameVerb = random();
    let noun: NameNoun = random();

    format!(
        "{} {}",
        tidy(verb.to_string()).to_capitalized(),
        tidy(noun.to_string()).to_capitalized()
    )
}

pub fn get_mood() -> String {
    let current_mood: MoodData = random();
    let result: String = match current_mood {
        MoodData::MerchantFriendly => "merchant-friendly".to_string(),
        _ => tidy(current_mood.to_string()),
    };
    result
}

pub fn get_lighting() -> String {
    let adjective: LightingAdjectives = random();
    let verb: LightingVerb = random();
    let source: LightingSources = random();
    let result = format!(
        " and the main area is {} {} by {}",
        tidy(adjective.to_string()),
        tidy(verb.to_string()),
        tidy(source.to_string())
    );
    result
}

pub fn get_smells() -> String {
    let sniff1: FirstSmell = random();
    let sniff2: SecondSmell = random();
    let result = format!(
        "{} and {}",
        tidy(sniff1.to_string()),
        tidy(sniff2.to_string())
    );

    result
}

pub fn get_posted_sign() -> String {
    let sign_location: PostedSignLocation = random();
    let sign_message: PostedSignMessage = random();

    let sign_location_text = tidy(sign_location.to_string())
        .replace("stags", "stag's")
        .replace("trophy mounted", "trophy-mounted");
    let percent_replace = format!(
        "({} {}!!)",
        DiceResult::from_string("4d4").get_total(),
        "percent off"
    );

    let sign_message_text: String = {
        if sign_message == PostedSignMessage::ColorfulNamesOfPriorGuests {
            let how_many_ban_hammers = DiceResult::from_string("2d4+1").get_total();
            format!("The following people are BANNED from this establishment!!! (A colorful list of {} name follows)", how_many_ban_hammers)
        } else {
            tidy(sign_message.to_string())
                .replace("dont", "don't")
                .replace("percent off", &percent_replace)
        }
    };

    let result = format!(
        " A sign {} says '{}'.",
        sign_location_text,
        sign_message_text.to_capitalized()
    );
    result
}

pub fn get_house_drink() -> HouseDrink {
    let desc: String = "HouseDrink Desc".to_string();
    let price: String = "11cp".to_string();
    HouseDrink { desc, price }
}

pub fn get_house_dish() -> HouseDish {
    let desc: String = "HouseDish Desc".to_string();
    let price: String = "13cp".to_string();
    HouseDish { desc, price }
}
// --- eof ---
