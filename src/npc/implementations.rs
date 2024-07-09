// ---- implementations ----
pub mod List {
    use crate::enums::List::*;
    use crate::structs::List::*;
    use crate::traits::List::*;
    use rand::distributions::{WeightedIndex,Distribution};
    use rand::thread_rng;
    use strum::{IntoEnumIterator,VariantArray};

    impl Tombstone<'_> {
        pub fn new() -> Self {
            Tombstone {
                char_type: TypeCodeList::Drifter,
                gender: GenderCodeList::Androgenous,
                partner_preference: PartnerPreferenceCodeList::Hetro,
                public_name: "Jane Q Publique",
                task_desc: "Wandering Wanderer",
                race: RaceCodeList::Human,
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
            let buffer = RaceCodeList::VARIANTS[table_weights.sample(&mut rng)];
            buffer
        }
    }

    // impl HasWeightedRandom for GenderCodeList {
    //     fn weighted_random() -> Self {
    //     }
    // }

} // pub mod List

// ---- end of file ----
