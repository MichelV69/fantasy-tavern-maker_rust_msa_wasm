#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use is_vowel::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand_derive2::RandGen;
use std::{cmp::*, fmt};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumString};

use crate::enums::List::*;
use crate::structs::List::*;
use crate::Tower::DiceResult;

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

pub fn get_pb_house_size() -> PBHouseSize {
    let pb_size: SizeList = random();
    let our_pbhouse: PBHouseSize = match pb_size {
        SizeList::Tiny => {
            let pb_tables_roll = DiceResult::from_string("2d4");
            let pb_tables = pb_tables_roll.get_total();

            let pb_beds_roll = DiceResult::from_string("1d4");
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
            let pb_tables_roll = DiceResult::from_string("3d4");
            let pb_tables = pb_tables_roll.get_total();

            let pb_beds_roll = DiceResult::from_string("2d4");
            let pb_beds = pb_beds_roll.get_total();

            let pb_priv_room_roll = DiceResult::from_string("1d4");
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
            let pb_tables_roll = DiceResult::from_string("4d6");
            let pb_tables = pb_tables_roll.get_total();

            let pb_beds_roll = DiceResult::from_string("3d6");
            let pb_beds = pb_beds_roll.get_total();

            let pb_priv_room_roll = DiceResult::from_string("2d6");
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
            let pb_tables_roll = DiceResult::from_string("5d6");
            let pb_tables = pb_tables_roll.get_total();

            let pb_beds_roll = DiceResult::from_string("4d6");
            let pb_beds = pb_beds_roll.get_total();

            let pb_priv_room_roll = DiceResult::from_string("3d6");
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
            let pb_tables_roll = DiceResult::from_string("7d8");
            let pb_tables = pb_tables_roll.get_total();

            let pb_beds_roll = DiceResult::from_string("6d8");
            let pb_beds = pb_beds_roll.get_total();

            let pb_priv_room_roll = DiceResult::from_string("4d8");
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

pub fn get_establishment_quality() -> EstablishmentQuality {
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

pub fn get_house_drink(eql: EstablishmentQualityLevel) -> HouseDrink {
    let weights_vector = (1..=DrinkMade::VARIANT_COUNT).collect::<Vec<usize>>(); // courtesy WGaffa (Twitch)
    let dist = WeightedIndex::new(weights_vector).unwrap();

    let mut rng = thread_rng();
    let options_list: Vec<_> = DrinkMade::iter().collect();
    let where_is_made = &options_list[dist.sample(&mut rng)];

    let drink_index: DrinkList = random();
    let drink_type_detail = match drink_index {
        DrinkList::Ales => drink_index.to_string(),
        DrinkList::Ciders => drink_index.to_string(),
        DrinkList::Whiskeys => drink_index.to_string(),
        DrinkList::Rums => drink_index.to_string(),
        DrinkList::Wines => drink_index.to_string(),
        DrinkList::OtherStock => drink_index.to_string(),
    };

    // description = $"{where_made} {PickFromList(drink_list)}";

    let desc: String = format!(" {} {}", where_is_made, drink_type_detail);
    let price: String = "11cp".to_string();
    HouseDrink { desc, price }
}

pub fn get_house_dish(eql: EstablishmentQualityLevel) -> HouseDish {
    let how_cooked: HouseDishHowCooked = random();
    let what_cooked: HouseDishWhatCooked = random();
    let side_dish: HouseDishWhatSide = random();

    let desc: String = format!(
        "{} {} served with {}",
        tidy(how_cooked.to_string()),
        tidy(what_cooked.to_string()),
        tidy(side_dish.to_string())
    );
    let cost_of_goods = get_cost_of_goods(eql);
    //coin_type, cost_minimum, dice_to_roll
    let roll_value = DiceResult::from_string(&cost_of_goods.2).get_total();
    let cost_of_goods_value = max(cost_of_goods.1, roll_value);

    let price: String = format!("{} {}", cost_of_goods_value, cost_of_goods.0);
    HouseDish { desc, price }
}

pub fn get_cost_of_goods(eql: EstablishmentQualityLevel) -> (String, i8, String) {
    let (coin_type, cost_minimum, dice_to_roll) = match eql {
        EstablishmentQualityLevel::Squalid => ("copper".to_string(), 2, "1d4+1".to_string()),
        EstablishmentQualityLevel::Poor => ("copper".to_string(), 3, "1d4+1".to_string()),
        EstablishmentQualityLevel::Modest => ("copper".to_string(), 15, "4d6+2".to_string()),
        EstablishmentQualityLevel::Comfortable => ("copper".to_string(), 20, "5d8+3".to_string()),
        EstablishmentQualityLevel::Wealthy => ("copper".to_string(), 30, "5d12+6".to_string()),
        EstablishmentQualityLevel::Aristocratic => ("silver".to_string(), 8, "2d6+2".to_string()),
    };
    (coin_type, cost_minimum, dice_to_roll)
}

// --- eof ---
