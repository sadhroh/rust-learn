#! [allow(unused)]

use std::{io, vec};
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
// fn main(){
//     let mut a = 1;
//     let arr = [1,2,3,4,5];

//     // iterate infinitely
//     loop{
//         // skip even values
//         if a % 2 == 0{
//             a += 1;
//             continue;
//         }
//         // stop when a exceeds 10
//         if a > 10{
//             break;
//         }
//         // print the value
//         println!("{}",a);
//         // increment value to next
//         a += 1;
//     }

//     // while loops
//     println!("While loops");
//     let mut i = 0;
//     while i < arr.len(){
//         println!("{}", arr[i]);
//         i+=1;
//     }

//     // for loops
//     println!("for loops");
//     for val in arr.iter(){
//         println!("{}", val);
//     }
//     for i in 1..10{
//         println!("Num:{}", i);
//     }
// }

// tuples
// fn main(){
//     let my_tup = (47, "John", 10_000);

//     println!("Name: {}", my_tup.1);

//     let (v1, v2, v3) = my_tup;
//     println!("Age: {}", v1);
// }

// strings
// fn main(){
//     // growable string
//     let mut st1 = String::new();

//     // push back a character
//     st1.push('A');
//     // push back a string
//     st1.push_str(" word");

//     // iterate over words
//     for word in st1.split_whitespace(){
//         println!("{}", word);
//     }

//     // replace strings
//     let st2 = st1.replace("A", "Another");
//     println!("{}", st2);

//     // creating a String with value
//     let st3 = String::from("x r h t k k a m c");

//     // convert to vector of characters
//     let mut v1: Vec<char> = st3.chars().collect();

//     // sort
//     v1.sort();
//     // remove duplicates
//     v1.dedup();

//     println!("Printing processed characters");
//     // iterate through the haracters
//     for char in v1{
//         println!("{}", char);
//     }

//     let st4 = "Random string";
    
//     // heap allocated string
//     let mut st5 = st4.to_string();
//     println!("Heap allocated string: {}", st5);

//     // string to array of bytes
//     let byte_arr = st5.as_bytes();
//     // slice of string
//     let st6 = &st5[0..6]; // 0 to 5 

//     println!("String length: {}", st6.len());

//     // delete values in a mutable string
//     st5.clear();
//     println!("After cearing: {}", st5);

//     // combine strings
//     let st6 = String::from("New String");
//     let st7 = String::from(" with words");
//     // gets reference to st7 but not st6, meaning that st6 is no longer available and exists within st8
//     let st8 = st6 + &st7;
//     // println!("Combined string: {} + {} --> {}", st6, st7, st8);
//     println!("Combined string: {}", st8);

//     // prints unicode characters
//     for char in st8.bytes(){
//         println!("{}", char);
//     }
// }

// casting
// fn main(){
//     let int1_u8: u8 = 5;
//     let int2_u8: u8 = 9;
//     let int3_u32: u32 = (int1_u8 as u32) + (int2_u8 as u32);

//     println!("Casted sum: {}", int3_u32);
// }

// enumerated types
// fn main(){
//     // allows custom data types having limited number of values
//     enum Day {
//         Monday,
//         Tuesday,
//         Wednesday,
//         Thursday,
//         Friday,
//         Saturday,
//         Sunday
//     }

//     // functions can be defined for the type
//     impl Day {
//         fn is_weekend(&self) -> bool{
//             match self {
//                 Day::Saturday | Day::Sunday => true,
//                 _ => false
//             }
//         }
//     }

//     let today = Day::Monday;

//     match today {
//         Day::Monday => println!("Monday"),
//         Day::Tuesday => println!("Tuesday"),
//         Day::Wednesday => println!("Wednesday"),
//         Day::Thursday => println!("Thursday"),
//         Day::Friday => println!("Friday"),
//         Day::Saturday => println!("Saturday"),
//         Day::Sunday => println!("Sunday"),
//     }

//     println!("Weekend tooday? {}", today.is_weekend());
// }

// vectors
// fn main(){
//     // declaring an empty vector
//     let v1: Vec<i32> = Vec::new();
//     // vector with values
//     let mut v2 = vec![1,2,3,4];
//     // add values to the end
//     v2.push(10);
//     // get at specific index
//     println!("{}", v2[2]);
//     // check if value exists
//     match v2.get(4) {
//         Some(&10) => println!("Matched {}", 10),
//         None => println!("Not found"),
//         _ => println!("NOt 10"),
//     };
//     // iterate and change values
//     for i in &mut v2{
//         *i *= 5;
//     }
//     for i in &v2{
//         println!("Value: {}", i);
//     }
//     // vector length
//     println!("Length: {}", v2.len());
//     // remove last value
//     println!("Last value: {:?}", v2.pop());
// }

// functions
fn main(){
    say_hello();

    sum(5, 10);
    
    println!("{}", get_sum(2, 4));
    
    println!("{}", get_back_sum(3, 6));

    let (sum, diff, prod) = get_sum_diff(6, 8);
    println!("Sum: {}, Diff: {}, Prod: {}", sum, diff, prod);

    let list = vec![1,2,3,4,5];
    println!("Sum of list: {}", sum_list(&list));
}
fn say_hello(){
    println!("Hello");
}
fn sum(x: i32, y: i32){
    println!("{} + {} = {}", x, y, x + y);
}
fn get_sum(x: i32, y: i32) -> i32{
    // return the expression
    // don't put ; as this statement doesn't evaluate to a function
    x + y
}
fn get_back_sum(x: i32, y: i32) -> i32{
    return x + y;
}
fn get_sum_diff(x: i32, y:i32) -> (i32, i32, i32){
    return (x+y, x-y, x*y);
}
fn sum_list(list: &[i32]) -> i32{
    let mut sum = 0;
    for i in list.iter(){
        sum += i;
    }
    return sum
}