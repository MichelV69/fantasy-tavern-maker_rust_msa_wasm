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

pub fn enum_to_text(s: String) -> String {
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

// ---
pub fn get_name() -> String {
    let verb: NameVerb = random();
    let noun: NameNoun = random();

    format!("'{} {}'", verb, noun)
}

pub fn get_mood() -> String {
    let current_mood: MoodData = random();
    let result = current_mood.to_string();
    trim_whitespace(enum_to_text(result))
}

pub fn get_lighting() -> String {
    let adjective: LightingAdjectives = random();
    let verb: LightingVerb = random();
    let source: LightingSources = random();
    let result = format!(" The main area is {} {} by {}", adjective, verb, source);
    trim_whitespace(enum_to_text(result))
}

pub fn get_smells() -> String {
    let sniff1: FirstSmell = random();
    let sniff2: SecondSmell = random();
    let result = format!("{} and {}", sniff1, sniff2);
    trim_whitespace(enum_to_text(result))
}

// --- eof ---
