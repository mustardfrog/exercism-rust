// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
        let a = expected_minutes_in_oven();
    let b = a - actual_minutes_in_oven;
    b
    
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
        let c = number_of_layers * 2;
    c
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    
        let d = number_of_layers * 2 ;
    let e = d + actual_minutes_in_oven;
    e
}
