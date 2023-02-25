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
fn main(){
    let rand_num: i32 = rand::thread_rng().gen_range(1..101);

    println!("Reandom num: {}", rand_num);
}