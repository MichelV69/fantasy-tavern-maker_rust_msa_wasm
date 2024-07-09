// --- enums
pub mod List {
    use strum::EnumIter;
    use strum::Display;
    use variant_count::VariantCount;
    use rand_derive2::RandGen;
    //#[derive(RandGen, Display, VariantCount, EnumIter)]

    #[derive(PartialEq, Debug)]
    pub enum TypeCodeList {
        Drifter,
        Staff,
        Patron,
        StoryCharacter,
    }

    #[derive(PartialEq, Debug)]
    pub enum GenderCodeList {
        Androgenous,
        Male,
        Female,
        NonBinary,
    }

    #[derive(PartialEq, Debug)]
    pub enum PartnerPreferenceCodeList {
        Hetro,
        GayLes,
        Bi,
        Asex,
    }

    #[derive(PartialEq, Debug)]
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
