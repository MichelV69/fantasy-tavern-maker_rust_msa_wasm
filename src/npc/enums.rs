// --- enums
pub mod List {
use strum::EnumIter;
use strum::Display;
use variant_count::VariantCount;
use rand_derive2::RandGen;

    enum TypeCodeList {
        Staff,
        Patron,
        StoryCharacter,
    }

    #[derive(RandGen, Display, VariantCount, EnumIter)]
    enum GenderCodeList {
        Male,
        Female,
        NonBinary,
    }

    //#[derive(RandGen, Display, VariantCount, EnumIter)]
    enum PartnerPreferenceCodeList {}

    //#[derive(RandGen, Display, VariantCount, EnumIter)]
    enum RaceCodeList {}
}
// ---- end of file ----
