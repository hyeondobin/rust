// // use std::io;
//
// fn main() {
//     // let a = [1, 2, 3, 4, 5];
//     //
//     // println!("Please enter an array index");
//     //
//     // let mut index = String::new();
//     //
//     // io::stdin()
//     //     .read_line(&mut index)
//     //     .expect("Failed to read line");
//     //
//     // let index: usize = index
//     //     .trim()
//     //     .parse()
//     //     .expect("Index entered was not a number");
//     //
//     // let element = a[index];
//     //
//     // println!("The value of the element at index {index} is: {element}");
//
//     // println!("main.");
//     // another_function(5);
//     //
//     // print_labeled_measurement(5, 'h');
//     // let y = 6;
//
//     // let x = (let y = 6);
//
//     // let y = {
//     //     let x = 3;
//     //     x + 1
//     // };
//     //
//     // println!("The value of y is: {y}");
//
//     let x = five();
//
//     println!("The value of x is: {x}");
// }
// //
// // fn another_function(x: i32) {
// //     println!("Another function.");
// //     println!("The value of x is: {x}");
// // }
// //
// // fn print_labeled_measurement(value: i32, unit_label: char) {
// //     println!("The measurement is: {value}{unit_label}");
// // }
//
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = plus_one(5);
//     print!("The value of x is: {x}");
// }
//
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// fn main() {
//     let x = plus_one(5);
//     print!("The value of x is: {x}");
// }
//
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// control flow

// fn main() {
//     let number = 3;
//
//     if number < 5 {
//         print!("condition was true");
//     } else {
//         print!("condition was false");
//     }
// }
//
// fn main() {
//     let number = 3;
//
//     if number != 0 {
//         print!("number was something other than zero");
//     }
// }
//
// fn main() {
//     let number = 6;
//
//     if number % 4 == 0 {
//         print!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         print!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         print!("number is divisible by 2");
//     } else {
//         print!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//
//     print!("The value of number is: {number}");
// }

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//
//         count += 1;
//     }
//     println!("End count = {count}");
//
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{number}");
//         number -= 1;
//     }
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;
//
//     while index < 5 {
//         println!("the value is: {}", a[index]);
//
//         index += 1;
//     }
// }

// for
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//
//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// for rev
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!");
}
