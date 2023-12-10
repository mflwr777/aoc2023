use std::fs;

fn main() {
    let filecontents = fs::read_to_string("/home/yurio/Desktop/aoc2023/advent_of_code/src/bin/input1")
        .expect("Error - was not able to read the contents of the file");
    //Intereting forusre!
    println!("{:?}", firstline);
}
