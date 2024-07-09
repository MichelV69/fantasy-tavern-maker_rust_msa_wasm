// --- enums
pub mod List {
    use strum::{Display,IntoEnumIterator};
    use strum_macros::{EnumIter,VariantArray};
    use variant_count::VariantCount;
    use rand_derive2::RandGen;
    //#[derive(RandGen, Display, VariantCount, EnumIter)]

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum TypeCodeList {
        Drifter,
        Staff,
        Patron,
        StoryCharacter,
    }

    #[derive(PartialEq, Debug, VariantCount, EnumIter, Clone, Copy)]
    pub enum GenderCodeList {
        Male,
        Female,
        NonBinary,
        Androgenous,
    }

    #[derive(PartialEq, Debug, EnumIter, VariantArray, Clone, Copy)]
    pub enum PartnerPreferenceCodeList {
        Hetro,
        GayLes,
        Bi,
        Asex,
    }

    #[derive(PartialEq, Debug, EnumIter, VariantArray, Clone, Copy)]
    pub enum RaceCodeList {
        Dragonborn,// 1
        Dwarf,// 5
        Elf,// 4
        Gnome,// 3
        HalfElf,// 2
        Halfling,// 4
        HalfOrc,// 1
        Human,// 7
        Tiefling,// 1
    }

    #[derive(PartialEq, Debug, EnumIter, VariantArray, Clone, Copy)]
    pub enum TaskDescList {
    Commonfolk, // 40 |
    Noble, // accompanied by [1d4] entourage, //  5 |
    Merchant, // accompanied by [1d4] guards, // 20 |
    Tinker, // 20 |
    Tailor, // 20 |
    Soldier, // 10 |
    Militiamun, // 20 |
    Spy, //  5 |
    Craftsperson, // 20 |
    Tradesperson, // 20 |
    Minstral, // 20 |
    BountyHunter, // accompanied by [1d4] thugs, //  5 |
    LayPriest, // 20 |
    UtilityMage, // (Mend, Daylight, Message, etc), //  5 |
    Courtesan, // accompanied by [1d4] entourage, //  5 |
    Adventurer, // ([1d4+1] level) with [1d4] henchmen ([1d4-1] level), //  1 |
    }

}
// ---- end of file ----
