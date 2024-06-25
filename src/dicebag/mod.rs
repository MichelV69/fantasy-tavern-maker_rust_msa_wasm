// ---- start of file ----
pub mod tower {
    use rand::distributions::Distribution;
    use rand::distributions::Uniform;
    use rand::prelude::*;
    use std::str::Split;
    use tracing::event;
    use tracing::Level;

    struct RollRequest {
        die_requested: DiceBag,
        number_rolls: i8,
        modifer_list: Vec<i8>,
    }

    enum DiceBag {
        Coin,
        D2,
        D4,
    }

    pub struct DiceResult {
        request: String,
        rolls: Vec<i8>,
        total_mod: i8,
        total_roll: i8,
    }
    trait GetDiceResult {
        fn get_total(&self) -> i8;
    }

    impl DiceResult {
        pub fn get_total(&self) -> i8 {
            self.total_roll
        }
        pub fn from_string(request: &str) -> DiceResult {
            let res1: Vec<String> = request.split('d').map(|s| s.to_string()).collect();
            let die_count = res1[0].parse::<i8>().unwrap();

            let res2: Vec<String> = res1[1].split("[-+]").map(|s| s.to_string()).collect();
            let die_size = &res2[0];

            let mut mod_list: Vec<i8> = [].to_vec();
            let stop = res2.len() - 1;
            if stop > 1 {
                event!(Level::INFO, "No modifiers passed");
                let res3: Vec<_> = res2[1..stop].to_vec();
                for (k, v) in res3.iter().enumerate() {
                    mod_list.push(v.parse::<i8>().unwrap());
                }
            }

            event!(
                Level::INFO,
                "die_size[{:#?}] die_count[{:#?}] mod_list[{:#?}]",
                die_size,
                die_count,
                mod_list
            );
            println!(
                "die_size[{:#?}] die_count[{:#?}] mod_list[{:#?}]",
                die_size, die_count, mod_list
            );

            let requested_size = match die_size.parse::<i8>().unwrap() {
                2 => DiceBag::Coin,
                4 => DiceBag::D4,
                _ => panic!("Unsupported die-size description."),
            };

            let new_roll_request = {
                RollRequest {
                    die_requested: requested_size,
                    number_rolls: die_count,
                    modifer_list: mod_list,
                }
            };

            let result = process_roll_request(new_roll_request);
            result
        }
    } // impl DiceResult

    fn process_roll_request(request: RollRequest) -> DiceResult {
        let mut rng = rand::thread_rng();
        let mut roll_list: Vec<i8> = [].to_vec();

        let die2 = Uniform::new(0, 2);
        let die4 = Uniform::new(0, 4);
        let die6 = Uniform::new(0, 6);
        let die8 = Uniform::new(0, 8);

        for index in 0..request.number_rolls {
            let roll_val: i8 = match request.die_requested {
                DiceBag::Coin | DiceBag::D2 => rng.sample::<i8, _>(die2) + 1,
                DiceBag::D4 => rng.sample::<i8, _>(die4) + 1,
            };
            event!(Level::INFO, "roll_val[{}]", roll_val);

            roll_list.push(roll_val);
        }

        let mut roll_total = 0;
        for roll in &roll_list {
            roll_total = roll_total + roll;
        }

        DiceResult {
            request: "no_roll".to_string(),
            rolls: roll_list,
            total_mod: 0,
            total_roll: roll_total,
        }
    }
} // pub mod dicebag{

// ---- start of tests ----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_flip_coin() {
        let request: String = "1d2".to_string();
        let resulting_roll = roll_string(&request);
        let roll_value: i8 = resulting_roll.total_roll;
        event!(Level::INFO, "roll_value[{}]", roll_value);

        debug_assert!(roll_value <= 2);
        debug_assert!(roll_value >= 1);
    }

    #[test]
    fn rolls_1d4() {
        let request: String = "1d4".to_string();
        let resulting_roll = roll_string(&request);
        let roll_value: i8 = resulting_roll.total_roll;
        event!(Level::INFO, "roll_value[{}]", roll_value);

        println!("roll_value[{}]", roll_value);

        debug_assert!(roll_value <= 4);
        debug_assert!(roll_value >= 1);
    }

    #[test]
    fn rolls_20d4() {
        let request: String = "2d4".to_string();
        let resulting_roll = roll_string(&request);
        let roll_value: i8 = resulting_roll.total_roll;
        event!(Level::INFO, "roll_value[{}]", roll_value);

        println!("roll_value[{}]", roll_value);
        for this_roll in resulting_roll.rolls {
            debug_assert!(this_roll <= 4);
            debug_assert!(this_roll >= 1);
        }
    }
}
// ---- end of file ----
