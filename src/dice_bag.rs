// ---- start of file ----
pub mod Tower {
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
        D6,
        D8,
        D10,
        D12,
        D20,
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
            // eg 5d12+6

            //5 d12+6
            let res1: Vec<String> = request.split('d').map(|s| s.to_string()).collect();
            let die_count = res1[0].parse::<i8>().unwrap();

            let res2: Vec<String> = res1[1]
                .replace('d', "")
                .split(&['+', '-'])
                .map(|s| s.to_string())
                .collect();

            let mut panic_reason = format!("no strings allowed '{:#?}'.", res2[0]);
            let die_size = &res2[0].parse::<i8>().expect(&panic_reason);

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

            let mut panic_reason = format!(
                "Unsupported die-size description c{} d{}  m'{:#?}'.",
                die_count, die_size, mod_list
            );
            let requested_size = match die_size {
                2 => DiceBag::Coin,
                4 => DiceBag::D4,
                6 => DiceBag::D6,
                8 => DiceBag::D8,
                10 => DiceBag::D10,
                12 => DiceBag::D12,
                20 => DiceBag::D20,
                _ => panic!("{}", &panic_reason),
            };

            let new_roll_request = {
                RollRequest {
                    die_requested: requested_size,
                    number_rolls: die_count,
                    modifer_list: mod_list,
                }
            };

            process_roll_request(new_roll_request)
        }
    } // impl DiceResult

    fn process_roll_request(request: RollRequest) -> DiceResult {
        let mut rng = rand::thread_rng();
        let mut roll_list: Vec<i8> = [].to_vec();

        let die2 = Uniform::new(0, 2);
        let die4 = Uniform::new(0, 4);
        let die6 = Uniform::new(0, 6);
        let die8 = Uniform::new(0, 8);
        let die10 = Uniform::new(0, 10);
        let die12 = Uniform::new(0, 12);
        let die20 = Uniform::new(0, 20);

        for index in 0..request.number_rolls {
            let low_roll_val: i8 = match request.die_requested {
                DiceBag::Coin | DiceBag::D2 => rng.sample::<i8, _>(die2),
                DiceBag::D4 => rng.sample::<i8, _>(die4),
                DiceBag::D6 => rng.sample::<i8, _>(die6),
                DiceBag::D8 => rng.sample::<i8, _>(die8),
                DiceBag::D10 => rng.sample::<i8, _>(die10),
                DiceBag::D12 => rng.sample::<i8, _>(die12),
                DiceBag::D20 => rng.sample::<i8, _>(die20),
            };
            let roll_val = low_roll_val + 1;
            event!(Level::INFO, "roll_val[{}]", roll_val);

            roll_list.push(roll_val);
        }

        let mut roll_total = 0;
        for roll in &roll_list {
            roll_total += roll;
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