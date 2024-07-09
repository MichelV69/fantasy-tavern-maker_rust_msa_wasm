// ---- implementations ----
pub mod List {
    use crate::enums::List::*;
    use crate::structs::List::*;
    use crate::traits::List::*;
    use rand::distributions::{WeightedIndex,Distribution};
    use rand::thread_rng;
    use strum::{IntoEnumIterator,VariantArray, VariantMetadata, EnumString};

    impl Tombstone<'_> {
        pub fn new() -> Self {
            Tombstone {
                char_type: TypeCodeList::Drifter,
                gender: GenderCodeList::Androgenous,
                partner_preference: PartnerPreferenceCodeList::Hetro,
                task_desc: TaskDescList::Commonfolk,
                race: RaceCodeList::Human,
                public_name: "Jane Q Publique",
                other_notes: vec![],
            }
        }
    }

    impl Getable for PartnerPreferenceCodeList {
        fn get_weight(self) -> i16 {
            match self {
                PartnerPreferenceCodeList::Hetro => 950,
                PartnerPreferenceCodeList::GayLes => 23,
                PartnerPreferenceCodeList::Bi => 16,
                PartnerPreferenceCodeList::Asex => 11,
            }
        }
    }

    impl Getable for RaceCodeList {
        fn get_weight(self) -> i16 {
            match self {
                RaceCodeList::Dragonborn => 1,
                RaceCodeList::Dwarf => 5,
                RaceCodeList::Elf => 4,
                RaceCodeList::Gnome => 3,
                RaceCodeList::HalfElf => 2,
                RaceCodeList::Halfling => 4,
                RaceCodeList::HalfOrc => 1,
                RaceCodeList::Human => 7,
                RaceCodeList::Tiefling => 1,
            }
        }
    }

    impl HasWeightedRandom for RaceCodeList {
        fn weighted_random() -> Self {
            let table_weights =
                WeightedIndex::new(RaceCodeList::iter().map(|item| item.get_weight())).unwrap();
            let mut rng = thread_rng();
            RaceCodeList::VARIANTS[table_weights.sample(&mut rng)]
        }
    }

    impl HasWeightedRandom for GenderCodeList {
        fn weighted_random() -> Self {
            let weights_vector = (1..=GenderCodeList::VARIANT_COUNT).collect::<Vec<usize>>(); // courtesy WGaffa (Twitch)
            let dist = WeightedIndex::new(weights_vector).unwrap();

            let mut rng = thread_rng();
            let options_list: Vec<_> = GenderCodeList::iter().collect();
            options_list[dist.sample(&mut rng)]
        }
    }

    impl HasWeightedRandom for PartnerPreferenceCodeList {
        fn weighted_random() -> Self {
            let table_weights =
                WeightedIndex::new(PartnerPreferenceCodeList::iter().map(|item| item.get_weight())).unwrap();
            let mut rng = thread_rng();
            PartnerPreferenceCodeList::VARIANTS[table_weights.sample(&mut rng)]
        }
    }

   impl Getable for TaskDescList {
        fn get_weight(self) -> i16 {
            match self {
                TaskDescList::Commonfolk => 40,
                TaskDescList::Noble => 5,//   accompanied by [1d4] entourage,
                TaskDescList::Merchant =>  20 , // accompanied by [1d4] guards,
                TaskDescList::Tinker => 20,
                TaskDescList::Tailor => 20,
                TaskDescList::Soldier => 10,
                TaskDescList::Militiamun => 20,
                TaskDescList::Spy => 5,
                TaskDescList::Craftsperson => 20,
                TaskDescList::Tradesperson => 20,
                TaskDescList::Minstral => 20,
                TaskDescList::BountyHunter => 5,  //accompanied by [1d4] thugs,
                TaskDescList::LayPriest => 20,
                TaskDescList::UtilityMage => 5,  //(can cast Mend, Daylight, Message, etc),
                TaskDescList::Courtesan => 5,  //accompanied by [1d4] entourage,
                TaskDescList::Adventurer => 1,  //([1d4+1] level) with [1d4] henchmen ([1d4-1] level),
            }
        }
    }
    impl HasWeightedRandom for TaskDescList {
        fn weighted_random() -> Self {
            let table_weights =
                WeightedIndex::new(TaskDescList::iter().map(|item| item.get_weight())).unwrap();
            let mut rng = thread_rng();
            TaskDescList::VARIANTS[table_weights.sample(&mut rng)]
        }
    }

    impl HasGetExtraNote for TaskDescList {
        fn get_other_note(task_desc: TaskDescList) -> &'static str {
            match task_desc {
                TaskDescList::Noble => "accompanied by [1d4] entourage",
                TaskDescList::Merchant =>  "accompanied by [1d4] guards",
                TaskDescList::BountyHunter => "accompanied by [1d4] thugs",
                TaskDescList::UtilityMage => "can cast Mend, Daylight, Message, etc",
                TaskDescList::Courtesan => "accompanied by [1d4] entourage",
                TaskDescList::Adventurer => "is [1d4+1] level with [1d4] henchmen of [1d4-1] level",
                _ => "",
            }.into()
        }
    }
} // pub mod List

// ---- end of file ----
