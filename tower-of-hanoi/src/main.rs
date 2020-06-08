use tower_of_hanoi::*;
use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();
    println!("How many discs will you move? ");
    let n_discs: i32 = stdin.lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    
    println!("Solving for {:?} discs...", n_discs);

    let (mut tower_a, mut tower_b, mut tower_c) = init(n_discs);

    hanoi(&mut tower_a, &mut tower_c, &mut tower_b, n_discs);

    println!("a{:?}\tb{:?}\tc{:?} \nend.", tower_a, tower_b, tower_c);
}
