// --- structs
pub mod List {
    use crate::enums::List::*;

    pub struct Tombstone<'a> {
        pub char_type: TypeCodeList,
        pub gender: GenderCodeList,
        pub partner_preference: PartnerPreferenceCodeList,
        pub public_name: &'a str,
        pub task_desc: TaskDescList,
        pub race: RaceCodeList,
        pub other_notes: Vec<&'a str>
    }

    pub struct RpDetails<'a> {
        pub height_desc: &'a str,
        pub build_desc: &'a str,
        pub hair_color: &'a str,
        pub hair_style: &'a str,
        pub eye_color: &'a str,
        pub quirk_emotional: &'a str,
        pub quirk_physical: &'a str,
        pub notable_attribute_positive: &'a str,
        pub notable_attribute_negative: &'a str,
        pub schtick_ability_description: &'a str,
    }

    pub struct Sheet<'a> {
        pub top: Tombstone<'a>,
        pub bottom: RpDetails<'a>,
    }
}
// ---- end of file ----
