#![allow(unused)]
const ONE_TO_FOUR :f64 = 1.0;
const FIVE_TO_EIGHT :f64 = 0.9;
const NINE_TO_TEN :f64 = 0.77;
const QUANTITY :f64 = 221.0;

pub fn calculate(speed: f64, rate: f64) -> f64 {
    return speed * QUANTITY * rate 
}
    
fn main(){}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let flt = speed as f64;
    match speed {
        1 => calculate(flt, ONE_TO_FOUR),
        2 => calculate(flt, ONE_TO_FOUR),
        3 => calculate(flt, ONE_TO_FOUR),
        4 => calculate(flt, ONE_TO_FOUR),
        5 => calculate(flt, FIVE_TO_EIGHT),
        6 => calculate(flt, FIVE_TO_EIGHT),
        7 => calculate(flt, FIVE_TO_EIGHT),
        8 => calculate(flt, FIVE_TO_EIGHT),
        9 => calculate(flt, NINE_TO_TEN),
        10 => calculate(flt, NINE_TO_TEN),
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let production = production_rate_per_hour(speed) as u32;
    let new_speed = speed as u32;

    return production / 60;

}

