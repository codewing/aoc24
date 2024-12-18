use std::{env, fs};

use movement::Movement;
use storage::Storage;

mod field;
mod movement;
mod storage;
mod utils;
mod vec;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_owned());
    let path = format!("{manifest_dir}/input/day15-challenge-input.txt");
    let content = fs::read_to_string(path).unwrap();
    let lines = content.split('\n');

    let (map_lines, movement_lines): (Vec<_>, Vec<_>) = lines
        .filter(|line| !line.is_empty())
        .partition(|line| line.contains('#'));

    let movements: Vec<_> = movement_lines
        .iter()
        .flat_map(|moves| moves.chars())
        .map(|m| Movement::try_from(m).unwrap())
        .collect();

    let mut storage = Storage::new(map_lines);

    println!("Initial State:");
    println!("{storage}");

    for movement in movements {
        storage.move_robot(&movement);
    }

    println!("---");
    println!("Final State:");
    println!("{storage}");
    let gps_sum = storage.calculate_gps_sum();
    println!("GPS Sum: {gps_sum}")
}
