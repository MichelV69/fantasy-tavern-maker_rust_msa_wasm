//--- start of file ---

// --- tests
#[cfg(test)]
mod tests {
    // create new Tombstone
    // assign char_type
    // randomize gender
    // randomize partner_preference
    // randomize public_name
    // randomize task_desc
    // randomize race
} // mod tests

pub mod Maker {
    use crate::RandGen;
    use strum::Display;
    use strum::EnumIter;
    use variant_count::VariantCount;

    // --- structs
    struct Tombstone<'a> {
        char_type: TypeCodeList,
        gender: GenderCodeList,
        partner_preference: PartnerPreferenceCodeList,
        public_name: &'a str,
        task_desc: &'a str,
        race: RaceCodeList,
    }

    struct RpDetails<'a> {
        height_desc: &'a str,
        build_desc: &'a str,
        hair_color: &'a str,
        hair_style: &'a str,
        eye_color: &'a str,
        quirk_emotional: &'a str,
        quirk_physical: &'a str,
        notable_attribute_positive: &'a str,
        notable_attribute_negative: &'a str,
        schtick_ability_description: &'a str,
    }

    struct Sheet<'a> {
        top: Tombstone<'a>,
        bottom: RpDetails<'a>,
    }

    // --- enums

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

    // --- implimentations

    // --- functions
} // pub mod Maker
  //--- end of file ---
