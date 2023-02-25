#! [allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// fn main() {
//     println!("Hello, world!");
// }

// accept user input
// fn main(){
//     let mut name:String = String::new();
//     let greeting:&str = "Welcome";
//     println!("Enter name: ");
//     io::stdin().read_line(&mut name).expect("Need input");
//     println!("Hello { }! {}", name.trim_end(), greeting);
// }

// variables and values
// fn main(){
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f32 = 3.14159265;

//     let age = "28";
//     let mut age: u32 = age.trim().parse()
//         .expect("Age not a num");

//     age += 1;

//     println!("Age {}. Data: {} PI:{}", age, ONE_MIL, PI);

//     println!("Max u32: {}", u32::MAX);
//     println!("Max usize: {}", usize::MAX);
//     println!("Max f32: {}", f32::MAX);
//     println!("Max i128: {}", i128::MAX);
// }

//math operations
// fn main(){
//     let num_1: u32 = 10;
//     let num_2: u32 = 7;

//     println!("{} + {} = {}", num_1, num_2, num_1 + num_2);
//     println!("{} - {} = {}", num_1, num_2, num_1 - num_2);
//     println!("{} * {} = {}", num_1, num_2, num_1 * num_2);
//     println!("{} / {} = {}", num_1, num_2, num_1 / num_2);
//     println!("{} % {} = {}", num_1, num_2, num_1 % num_2);
// }

// random num generator between 1 to 100
// fn main(){
//     let rand_num: i32 = rand::thread_rng().gen_range(1..101);

//     println!("Reandom num: {}", rand_num);
// }

// conditional statements
// fn main(){
//     let age = 18;

//     if (age >=1) && (age <= 18) {
//         println!("Teen");
//     } else if (age == 21) || (age == 50) {
//         println!("Important birthday");
//     } else {
//         println!("Not an important birthday");
//     }

//     // ternary
//     let mut my_age = 25;
//     let can_vote = if my_age >= 18 {
//         true
//     } else{
//         false
//     };
//     println!("Can vote: {}", can_vote);

//     // match
//     let age = 18;
//     match age {
//         1..=18=> println!("Can vote"),
//         21 | 50 => println!("Important birthday"),
//         65..=i32::MAX => println!("Senior"),
//         _ => println!("Not an important birthday"),
//     };

//     let my_age = 18;
//     let voting_age = 18;
//     match my_age.cmp(&voting_age) {
//         Ordering::Less => println!("Can't vote"),
//         Ordering::Greater => println!("Can vote"),
//         Ordering::Equal => println!("Welcome new voter"),
//     };
// }

// array
// fn main(){
//     let arr = [1,2,3,4,5];
//     println!("At index 0: {}", arr[0]);
//     println!("Len: {}", arr.len());
// }

// loop
fn main(){
    let mut a = 1;
    let arr = [1,2,3,4,5];

    // iterate infinitely
    loop{
        // skip even values
        if a % 2 == 0{
            a += 1;
            continue;
        }
        // stop when a exceeds 10
        if a > 10{
            break;
        }
        // print the value
        println!("{}",a);
        // increment value to next
        a += 1;
    }

    // while loops
    println!("While loops");
    let mut i = 0;
    while i < arr.len(){
        println!("{}", arr[i]);
        i+=1;
    }

    // for loops
    println!("for loops");
    for val in arr.iter(){
        println!("{}", val);
    }
    for i in 1..10{
        println!("Num: {}", i);
    }
}