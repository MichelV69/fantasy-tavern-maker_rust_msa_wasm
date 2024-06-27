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

pub fn tidy(s: String) -> String {
    trim_whitespace(enum_string_to_phase(s))
}

// ---
pub fn get_name() -> String {
    let verb: NameVerb = random();
    let noun: NameNoun = random();

    format!("'{} {}'", tidy(verb.to_string()), tidy(noun.to_string()))
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
    let result = format!(
        " The main area is {} {} by {}",
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

    let result = format!(
        " A sign {} says '{}'.",
        tidy(sign_location.to_string()),
        tidy(sign_message.to_string())
    );
    result
}

// --- eof ---
