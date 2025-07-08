// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("Cond was met");
//     } else {
//         println!("Cond was not met");
//     }
// }

// fn main() {
//     let number = 3;

//     // NOTE: rust will not just convert to bool for you. You must do it yourself.
//     if number >= 0 {
//         println!("Number: {number}")
//     }
// }

// ELIF
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// Let branching
// fn main() {
//     let condition = true;
//     let number: i32 = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// Returning from loops
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("final result: {result}");
// }

// // while
// fn main() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{number}");
//         number -= 1;
//     }

//     println!("LIFTOFF!!!!");
// }

// for loop

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     for element in a {
//         println!("The value is: {element}")
//     }
// }

// range + rev

fn main() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
}
