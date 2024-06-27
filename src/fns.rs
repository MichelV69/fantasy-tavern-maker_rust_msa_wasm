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

mod enums;
use enums::*;

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

pub fn tidy() -> (s: String) -> String {
     trim_whitespace(enum_string_to_phase(s))
}

// ---
pub fn get_name() -> String {
    let verb: NameVerb = random();
    let noun: NameNoun = random();

    format!("'{} {}'", verb, noun)
}

pub fn get_mood() -> String {
    let current_mood: MoodData = random();
    let result = current_mood.to_string();
    tidy(result)
}

pub fn get_lighting() -> String {
    let adjective: LightingAdjectives = random();
    let verb: LightingVerb = random();
    let source: LightingSources = random();
    let result = format!(" The main area is {} {} by {}", adjective, verb, source);
    tidy(result)
}

pub fn get_smells() -> String {
    let sniff1: FirstSmell = random();
    let sniff2: SecondSmell = random();
    let result = format!("{} and {}", trim_whitespace(enum_string_to_phase(sniff1)), trim_whitespace(enum_string_to_phase(sniff2)));

    result
}

pub fn get_posted_sign() -> String {
    let sign_location: PostedSignLocation = random();
    let sign_message: PostedSignMessage = random();

    let result = format!(" A sign {} says '{}'.", trim_whitespace(enum_string_to_phase(sign_location)), trim_whitespace(enum_string_to_phase(sign_message)));
}

// --- eof ---
