use std::fs;

use pgr2junit::pgr_model::Model;


#[test]
fn test1() {
    let a : Model;
    println!("dd: {}",fs::read_to_string("Cargo.toml").unwrap());
    // assert!(fs::read_to_string("Cargo.toml").unwrap().contains("x"));

    // assert_eq!(Solution::count_time(String::from("?5:00")),2);
}
