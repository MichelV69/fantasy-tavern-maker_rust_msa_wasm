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

    #[derive(PartialEq, Debug)]
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
}
// ---- end of file ----
