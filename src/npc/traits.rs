// ---- traits  ----
pub mod List {
    pub trait Getable {
        fn get_weight(self)  -> i16;
    }

    pub trait HasWeightedRandom {
        fn weighted_random() -> Self;
    }
}

// ---- end of file ----
