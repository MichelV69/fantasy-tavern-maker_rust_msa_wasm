// ---- start of file ----
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::str::Split;
use tracing::error;

struct RollRequest {
    die_requested: DiceBag,
    number_rolls: i8,
    modifer_list: Vec<i8>,
}

pub struct DiceResult {
    request: String,
    rolls: Vec<i8>,
    total_mod: i8,
    total_roll: i8,
}

enum DiceBag {
    Coin,
    D2,
    D4,
}

pub fn roll_string(request: &str) -> DiceResult {
    let res1: Vec<String> = request.split('d').map(|s| s.to_string()).collect();
    let die_count = res1[0].parse::<i8>().unwrap();

    let res2: Vec<String> = res1[1].split("[-+]").map(|s| s.to_string()).collect();
    let die_size = &res2[0];

    let stop = res2.len() - 1;
    let res3: Vec<_> = res2[1..stop].to_vec();
    let mut mod_list: Vec<i8> = [].to_vec();
    for (k, v) in res3.iter().enumerate() {
        mod_list.push(v.parse::<i8>().unwrap());
    }

    let new_roll_request = {
        RollRequest {
            die_requested: DiceBag::Coin,
            number_rolls: die_count,
            modifer_list: mod_list,
        }
    };

    // todo!("write roll processor now that we have formatted request");
    process_roll_request(new_roll_request)
}

fn process_roll_request(request: RollRequest) -> DiceResult {
    let mut rng = rand::thread_rng();
    let mut roll_list: Vec<i8> = [].to_vec();

    let die2 = Uniform::from(1..2);
    let die4 = Uniform::from(1..4);
    let die6 = Uniform::from(1..6);
    let die8 = Uniform::from(1..8);

    for index in 1..request.number_rolls {
        let roll_val: i8 = match request.die_requested {
            DiceBag::Coin | DiceBag::D2 => die2.sample(&mut rng),
            DiceBag::D4 => die4.sample(&mut rng),
            _ => panic!("uknown die type"),
        };
        roll_list.push(roll_val);
    }

    DiceResult {
        request: "".to_string(),
        rolls: [0].to_vec(),
        total_mod: 0,
        total_roll: -1,
    }
}
// ---- end of file ----
