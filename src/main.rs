use std::fs;
use fs::read_to_string;


mod day1;
mod day2;
use day1::{getFloor, getFirstBasementValue};
use day2::calculateSqFeetSum;

fn main() {


}

fn day2(){ //Not complete!
    let input : Result<String, _> = read_to_string("src/inputs/day2Input.txt");
    let binding : String = input.unwrap();
    calculateSqFeetSum(binding);
}
fn day1(){
    //note to self - learn how to clone/copy rather than borrowing
    let input : Result<String, _> = read_to_string("src/inputs/day1Input.txt");
    let task1 = getFloor(input.unwrap());
    let task2 = getFirstBasementValue(task1.1);
    println!("The answer to task 1 is {}, \nThe answer to task 2 is {}", task1.0, task2);
}
