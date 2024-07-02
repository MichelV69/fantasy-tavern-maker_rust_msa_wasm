#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use crate::Tower;
use crate::Tower::*;

use inflector::string::singularize::to_singular;
use is_vowel::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand_derive2::RandGen;
use std::{cmp::*, fmt};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumString};

use crate::enums::List::*;
use crate::structs::List::*;

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
        <Tower::DiceResult as RollDice>::from_string("4d4").get_total(),
        "percent off"
    );

    let sign_message_text: String = {
        if sign_message == PostedSignMessage::ColorfulNamesOfPriorGuests {
            let how_many_ban_hammers =
                <Tower::DiceResult as RollDice>::from_string("2d4+1").get_total();
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
            let pb_tables_roll = <Tower::DiceResult as RollDice>::from_string("2d4");
            let pb_tables = pb_tables_roll.get_total();

            let pb_beds_roll = <Tower::DiceResult as RollDice>::from_string("1d4");
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
            let pb_tables_roll = <Tower::DiceResult as RollDice>::from_string("3d4");
            let pb_tables = pb_tables_roll.get_total();

            let pb_beds_roll = <Tower::DiceResult as RollDice>::from_string("2d4");
            let pb_beds = pb_beds_roll.get_total();

            let pb_priv_room_roll = <Tower::DiceResult as RollDice>::from_string("1d4");
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
            let pb_tables_roll = <Tower::DiceResult as RollDice>::from_string("4d6");
            let pb_tables = pb_tables_roll.get_total();

            let pb_beds_roll = <Tower::DiceResult as RollDice>::from_string("3d6");
            let pb_beds = pb_beds_roll.get_total();

            let pb_priv_room_roll = <Tower::DiceResult as RollDice>::from_string("2d6");
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
            let pb_tables_roll = <Tower::DiceResult as RollDice>::from_string("5d6");
            let pb_tables = pb_tables_roll.get_total();

            let pb_beds_roll = <Tower::DiceResult as RollDice>::from_string("4d6");
            let pb_beds = pb_beds_roll.get_total();

            let pb_priv_room_roll = <Tower::DiceResult as RollDice>::from_string("3d6");
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
            let pb_tables_roll = <Tower::DiceResult as RollDice>::from_string("7d8");
            let pb_tables = pb_tables_roll.get_total();

            let pb_beds_roll = <Tower::DiceResult as RollDice>::from_string("6d8");
            let pb_beds = pb_beds_roll.get_total();

            let pb_priv_room_roll = <Tower::DiceResult as RollDice>::from_string("4d8");
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
    let drink_type_group = match drink_index {
        DrinkList::Ales => drink_index.to_string(),
        DrinkList::Ciders => drink_index.to_string(),
        DrinkList::Whiskeys => drink_index.to_string(),
        DrinkList::Rums => drink_index.to_string(),
        DrinkList::Wines => drink_index.to_string(),
        DrinkList::OtherStock => drink_index.to_string(),
    };

    let drink_type_detail: String = match drink_index {
        DrinkList::Ales => {
            let buffer: DrinkAlesDetail = random();
            tidy(buffer.to_string())
        }
        DrinkList::Ciders => {
            let buffer: DrinkCidersDetail = random();
            tidy(buffer.to_string())
        }
        DrinkList::Whiskeys => {
            let buffer: DrinkWhiskeysDetail = random();
            tidy(buffer.to_string())
        }
        DrinkList::Rums => {
            let buffer: DrinkRumsDetail = random();
            tidy(buffer.to_string())
        }
        DrinkList::Wines => {
            let buffer: DrinkWinesDetail = random();
            tidy(buffer.to_string())
        }
        DrinkList::OtherStock => {
            let options_list = [
                "Gin".to_string(),
                "Clearfire".to_string(),
                "a thick black liqueur brewed with herbs from the local area".to_string(),
                "a milky liqueur that closely resembles heavy cream".to_string(),
                format!(
                    "an iced cocktail made with {} different liquers",
                    <Tower::DiceResult as RollDice>::from_string("1d2+1").get_total()
                ),
                format!(
                    "a coffee-based drink, served in a stien, with {} strong spirits mixed in",
                    <Tower::DiceResult as RollDice>::from_string("1d2+1").get_total()
                ),
            ];
            let mut rng = thread_rng();
            let result = options_list
                .choose(&mut rng)
                .expect("valid roll in range")
                .to_string();
            result
        }
    };
    let desc: String = format!(
        "{} {} {}",
        tidy(where_is_made.to_string()).replace("houses", "House's"),
        drink_type_detail,
        to_singular(&drink_type_group)
    )
    .replace(" OtherStock", "");

    let cost_of_goods = get_cost_of_goods(eql);
    let roll_value = <Tower::DiceResult as RollDice>::from_string(&cost_of_goods.2).get_total();
    let cost_of_goods_value = max(cost_of_goods.1, roll_value);
    let price: String = format!("{} {}", cost_of_goods_value, cost_of_goods.0);

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
    let roll_value = <Tower::DiceResult as RollDice>::from_string(&cost_of_goods.2).get_total();
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

pub fn get_establishment_history_age() -> String {
    let weights_vector = (1..=EstablishmentHistoryAge::VARIANT_COUNT).collect::<Vec<usize>>(); // courtesy WGaffa (Twitch)
    let dist = WeightedIndex::new(weights_vector).unwrap();

    let mut rng = thread_rng();
    let options_list: Vec<_> = EstablishmentHistoryAge::iter().collect();
    let chosen_option = &options_list[dist.sample(&mut rng)];

    match chosen_option {
        EstablishmentHistoryAge::Generational => <Tower::DiceResult as RollDice>::inline_replace(
            "recently established, within the past [2d4+2] months",
        ),
        EstablishmentHistoryAge::Permanent => <Tower::DiceResult as RollDice>::inline_replace(
            "well established, and has been here for [4d4+3] months",
        ),
        EstablishmentHistoryAge::WellEstablished => {
            <Tower::DiceResult as RollDice>::inline_replace(
                "a permanent local fixture, and has been in business for [2d6] years",
            )
        }
        EstablishmentHistoryAge::Recent => <Tower::DiceResult as RollDice>::inline_replace(
            "a multi-generation business, in operation for more than [3d8+12] years",
        ),
    }
}

pub fn get_establishment_appearance() -> String {
    let weights_vector = (1..=EstablishmentAppearance::VARIANT_COUNT).collect::<Vec<usize>>(); // courtesy WGaffa (Twitch)
    let dist = WeightedIndex::new(weights_vector).unwrap();

    let mut rng = thread_rng();
    let options_list: Vec<_> = EstablishmentAppearance::iter().collect();
    let chosen_option = &options_list[dist.sample(&mut rng)];

    match chosen_option {
        EstablishmentAppearance::MinorRepairs => {
            "The building is in need of minor repairs to the exterior"
        }
        EstablishmentAppearance::GoodCondition => {
            "The building is in good condition, and shows evidence of regular care"
        }
        EstablishmentAppearance::BrandNew => {
            "The establishment and its grounds look to be nearly brand new"
        }
        EstablishmentAppearance::WhiteWashed => {
            "Parts of the exterior are fire-blacked or white-washed, looking recent"
        }
    }
    .into()
}

pub fn get_establishment_reputation() -> String {
    let weights_vector = (1..=EstablishmentReputuation::VARIANT_COUNT).collect::<Vec<usize>>(); // courtesy WGaffa (Twitch)
    let dist = WeightedIndex::new(weights_vector).unwrap();

    let mut rng = thread_rng();
    let options_list: Vec<_> = EstablishmentReputuation::iter().collect();
    let mut chosen_option = &options_list[dist.sample(&mut rng)];

    chosen_option = match chosen_option {
        EstablishmentReputuation::MurderScene => &options_list[dist.sample(&mut rng)],
        _ => chosen_option,
    };

    let result = match chosen_option {
        EstablishmentReputuation::PlotRumors => "Owner knows plot-relevant rumors",
        EstablishmentReputuation::MerchantsLike => "Traveling merchants know the place well",
        EstablishmentReputuation::MilitaPatrol => &<Tower::DiceResult as RollDice>::inline_replace(
            "A local milita band stops by here every [3d6+4] days as part of their patrol route",
        ),
        EstablishmentReputuation::MurderScene => "An infamous murder happened here",
    };

    result.into()
}

pub fn get_red_light_services_list() -> Option<String> {
    if <Tower::DiceResult as RollDice>::from_string("flip coin").get_total() == 2 {
        return None;
    }

    struct ServiceTableItem<'a> {
        weight: i8,
        description: &'a str,
        dc_dice_roll: &'a str,
    }

    let mut possible_services_table = Vec::<ServiceTableItem>::with_capacity(8);
    possible_services_table.push(ServiceTableItem {
        weight: 5,
        description: "Gambling",
        dc_dice_roll: "1d4+8",
    });
    possible_services_table.push(ServiceTableItem {
        weight: 4,
        description: "Brothel Services",
        dc_dice_roll: "1d4+10",
    });
    possible_services_table.push(ServiceTableItem {
        weight: 3,
        description: "Smuggling",
        dc_dice_roll: "2d4+11",
    });
    possible_services_table.push(ServiceTableItem {
        weight: 2,
        description: "Pit Fighting",
        dc_dice_roll: "2d4+12",
    });
    possible_services_table.push(ServiceTableItem {
        weight: 1,
        description: "Sinfyre Den",
        dc_dice_roll: "3d4+13",
    });
    possible_services_table.push(ServiceTableItem {
        weight: 1,
        description: "Thief / Assassin Guild (ADV w/Thieves Cant)",
        dc_dice_roll: "4d4+14",
    });

    let how_many_services = <Tower::DiceResult as RollDice>::from_pool("6d6|6");
    Some(format!(
        "{} services rendered via {}",
        how_many_services.get_total(),
        how_many_services.get_request()
    ))
}

// --- eof ---
