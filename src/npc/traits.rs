// ---- traits  ----
pub mod List {
    use crate::enums::List::*;

    pub trait Getable {
        fn get_weight(self)  -> i16;
    }

    pub trait HasWeightedRandom {
        fn weighted_random() -> Self;
    }

    pub trait HasGetExtraNote{
        fn get_other_note(task_desc: TaskDescList) -> &'static str;
    }
}

// ---- end of file ----
