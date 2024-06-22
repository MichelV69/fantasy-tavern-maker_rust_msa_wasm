// ---
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use strum_macros::Display;
use strum_macros::EnumString;

use is_vowel::*;
use rand::prelude::*;
use rand_derive2::RandGen;

use std::fmt;

#[derive(RandGen, Display)]
pub enum NameVerb {
    Waltzing,
    Checkered,
    Lazy,
    Silver,
    Saucy,
    Flirting,
    Blue,
    Red,
    Green,
    Yellow,
    Fickle,
    Roaring,
    Carousing,
    Melting,
    Drifting,
    Spring,
    Winter,
    Summer,
    Autumn,
    Pouring,
    Heaving,
}

#[derive(RandGen, Display)]
pub enum NameNoun {
    Werebear,
    Cockrel,
    Hen,
    Dragon,
    Wench,
    Dryad,
    Sky,
    Tide,
    Meadow,
    Sun,
    Fortune,
    Waters,
    Bard,
    Curmudgeon,
    Crystal,
    Mongrel,
    Ice,
    Tempest,
    Snows,
    Draft,
    Harvest,
    Chalice,
    Waves,
}

// ---

#[derive(Debug, RandGen, Display, EnumString, Eq, PartialEq)]
pub enum MoodData {
    Jovial,
    Relaxing,
    Smoky,
    Erudite,
    Loud,
    Subdued,
    Rowdy,
    Seedy,
    Shady,
    Busy,
    LowerClass,
    MiddleClass,
    UpperClass,
    MerchantFriendly,
    Dour,
    Flirty,
}
// ----
#[derive(Display, RandGen)]
pub enum LightingAdjectives {
    Brightly,
    Clearly,
    Evenly,
    Dimly,
    Shadowly,
}
#[derive(Display, RandGen)]
pub enum LightingVerb {
    Lit,
    Illuminated,
}
#[derive(Display, RandGen)]
pub enum LightingSources {
    Candles,
    AFireplace,
    OilLamps,
    MagicOrbsAndCrystals,
}
