// --- structs
pub mod List {
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
}
// ---- end of file ----
