// --- structs
pub mod List {
    use crate::enums::List::*;

    pub struct Tombstone<'a> {
        pub char_type: TypeCodeList,
        pub gender: GenderCodeList,
        pub partner_preference: PartnerPreferenceCodeList,
        pub public_name: &'a str,
        pub task_desc: &'a str,
        pub race: RaceCodeList,
    }

    pub struct RpDetails<'a> {
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

    pub struct Sheet<'a> {
        top: Tombstone<'a>,
        bottom: RpDetails<'a>,
    }
}
// ---- end of file ----
