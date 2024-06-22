// ---- start of file ----
use tracing::error;
use std::str::Split;

struct RollRequest {
    die_requested : DiceBag,
    number_rolls : i8,
    modifer_list : Vec<i8>
}

struct DiceResult {
    request : String,
    rolls : Vec<i8>,
    total_mod : i8,
    total_roll : i8,
}

enum DiceBag {
    Coin,
    D2,
    D4,
}

pub fn roll_string(request :  &str) -> String {
    let res1: Vec<String> = request.split('d').map(|s| s.to_string()).collect();
    let die_count  = res1[0].parse::<i8>().unwrap();

    let res2: Vec<String> = res1[1].split("[-+]").map(|s| s.to_string()).collect();
    let die_size = &res2[0];

    let stop = res2.len()-1;
    let res3 : Vec<_> = res2[1..stop].to_vec();
    let mut mod_list : Vec<i8> = [].to_vec();
    for (k, v) in res3.iter().enumerate() {
        mod_list.push(v.parse::<i8>().unwrap());
    }

    let new_roll_request = { RollRequest{die_requested : DiceBag::Coin, number_rolls: die_count, modifer_list: mod_list}};

    todo!("write roll processor now that we have formatted request");
    "bork".to_string()
}
// ---- end of file ----
