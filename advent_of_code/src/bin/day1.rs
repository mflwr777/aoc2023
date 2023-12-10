use std::fs::read_to_string;



fn main() {
    let filecontents = std::fs::read_to_string("/home/yurio/Desktop/aoc2023/advent_of_code/src/bin/input1")
        .expect("Error - was not able to read the contents of the file");
    let lines = &filecontents.lines();
    let lines_collections: Vec<&str> = lines.clone().collect();

    let mut first_int_collections: Vec<i32> = Vec::new();
    let mut second_int_collectios: Vec<i32> = Vec::new();

    for line in lines_collections.iter() {
        for ch in line.chars() {
            if ch.is_digit(10) {  // Check if the character is a digit
                if let Ok(num) = ch.to_string().parse::<i32>() {
                    first_int_collections.push(num);
                    break; // Break the inner loop once a character is successfully parsed as an integer
                } else {
                    // Other stuff here in case we wanted to do something else than break out of it 
                }
            }
        }
    }

    for line in lines_collections.iter() {
        for ch in line.chars().rev() {
            if ch.is_digit(10) {  // Check if the character is a digit
                if let Ok(num) = ch.to_string().parse::<i32>() {
                    second_int_collectios.push(num);
                    break; // Break the inner loop once a character is successfully parsed as an integer
                } else {
                    // Other stuff here in case we wanted to do something else than break out of it 
                }
            }
        }
    }

    println!("First int collection: {:?}, Second int collection: {:?}",first_int_collections, second_int_collectios);


    let mut combined_values: Vec<String> = first_int_collections.clone().iter().zip(second_int_collectios.iter()).map(|(&x,&y)| x.to_string() + &y.to_string()).collect();

    let sum: i32 = combined_values.iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .sum();


    println!("The combined values look like: {:?}", combined_values);
    println!("The sum of the calibration values is: {:?}", sum)


    //Sucess!
}




//ChatGPT suggested:

// use std::fs;

// fn main() {
//     let filecontents = fs::read_to_string("/home/yurio/Desktop/aoc2023/advent_of_code/src/bin/input1")
//         .expect("Error reading the file");
//     let lines = filecontents.lines();

//     let first_int_collections: Vec<i32> = lines.clone().map(|line| find_first_digit(line)).collect();
//     let second_int_collections: Vec<i32> = lines.map(|line| find_first_digit_reverse(line)).collect();

//     let combined_values: Vec<String> = first_int_collections.iter()
//         .zip(second_int_collections.iter())
//         .map(|(&x, &y)| format!("{}{}", x, y))
//         .collect();

//     let sum: i32 = combined_values.iter()
//         .filter_map(|s| s.parse().ok())
//         .sum();

//     println!("Combined values: {:?}", combined_values);
//     println!("Sum: {}", sum);
// }

// fn find_first_digit(line: &str) -> i32 {
//     line.chars()
//         .find_map(|ch| ch.to_digit(10).and_then(|num| Some(num as i32)))
//         .unwrap_or(0) // Handle the case where no digit is found
// }

// fn find_first_digit_reverse(line: &str) -> i32 {
//     line.chars()
//         .rev()
//         .find_map(|ch| ch.to_digit(10).and_then(|num| Some(num as i32)))
//         .unwrap_or(0) // Handle the case where no digit is found
// }

