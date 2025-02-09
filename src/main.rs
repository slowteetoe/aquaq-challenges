mod challenge_0;
mod challenge_1;
mod challenge_10;
mod challenge_11;
mod challenge_14;
mod challenge_15;
mod challenge_17;
mod challenge_2;
mod challenge_3;
mod challenge_32;
mod challenge_4;
mod challenge_5;
mod challenge_6;
mod challenge_7;
mod challenge_8;
mod challenge_9;

// ya, at some point this just looks ridiculous... maybe a macro instead?
fn main() {
    println!("challenge 0 -> {}", challenge_0::solve());
    println!("challenge 1 ->{}", challenge_1::solve());
    println!("challenge 2 ->{}", challenge_2::solve());
    println!("challenge 3 ->{}", challenge_3::solve());
    println!("challenge 4 ->{}", challenge_4::solve());
    println!("challenge 5 ->{}", challenge_5::solve());
    println!("challenge 6 ->{}", challenge_6::solve());
    println!("challenge 7 ->{}", challenge_7::solve());
    println!("challenge 8 ->{}", challenge_8::solve());
    println!("challenge 9 ->{}", challenge_9::solve());
    println!("challenge 10 ->{}", challenge_10::solve());
    println!("challenge 11 ->{}", challenge_11::solve());
    println!("challenge 14 ->{}", challenge_14::solve());
    println!("challenge 15 ->{}", challenge_15::solve());
    println!("challenge 17 ->{:?}", challenge_17::solve());
    println!("challenge 32 ->{}", challenge_32::solve());
}
