#![allow(unused)]

// use rand::Rng;
// use std::cmp::Ordering;
// use std::fs::File;
// use std::io::{BufRead, BufReader, ErrorKind, Write};
// use std::{io, string};

// fn main() {
// simple read and write example ::
// println!("enter name :");
// let mut name = String::new();
// let greeting = "nice to meet you";
// io::stdin()
//     .read_line(&mut name)
//     .expect("didnt receive input");
// println!("Hello {} {}", name.trim_end(), greeting);

// simple variables and consts example ::
// consts
// const ONE_MIL: u32 = 1_000_000;
// const PI: f32 = 3.141592;
// // variables
// // let age: &str = "47";
// let age = "47";
// // let mut age: u32 = 47;
// let mut age: u32 = age.trim().parse().expect("age wasn't assiged a number");
// age = age + 1;
// println!("age == {} one mil {}", age, ONE_MIL);

// data types ::
// unsigned integers : u8, u16, u32, u64, u128, usize
// signed integers : i8, i16, i32, i64, i128, isize
// println!("max u16 {}", u16::MAX);
// println!("max i128 {}", i128::MAX);
// let is_true = true;
// let is_true: bool = false;
// rand
// let rand_num = rand::thread_rng().gen_range(1..=100);
// println!("random value {}", rand_num);
// conditional
// if (true) && (false) || (true || false) {
//     println!("nice");
// }
// let num1 = if true { 10 } else { 20 };
// let age = 23;
// let can_vote = if age >= 18 { true } else { false };
// let age2 = 13;
// match age2 {
//     6..=9 => println!("hmm"),
//     10..=13 => println!("hmm hmm"),
//     14..=16 => println!("test"),
//     17.. => println!("blah"),
//     _ => println!("what"),
// }
// match age2.cmp(&age) {
//     Ordering::Less => println!("good"),
//     Ordering::Equal => println!("ehh"),
//     Ordering::Greater => println!("nah"),
// }

// loops ::
// let arr = [1, 2, 3, 4, 5];
// println!("index :: 0 == {}", arr[0]);
// println!("len  == {}", arr.len());

// let mut index = 0;
// loop {
//     if index == arr.len() {
//         break;
//     }
//     if arr[index] % 2 == 0 {
//         index += 1;
//         continue;
//     }
//     println!("val : {}", arr[index]);
//     index += 1;
// }
// index = 0;
// while index < arr.len() {
//     println!("{}", arr[index]);
// }
// for val in arr.iter() {
//     println!("val : {}", val);
// }

// tuple ::
// let tuple1: (u8, String, f64) = (13, "best".to_string(), 6.9);
// println!("{}", tuple1.0);
// println!("{}", tuple1.1);
// println!("{}", tuple1.2);

// strings ::
// let mut st1 = String::new();
// // let x: u32 = 10;
// st1.push('o');
// st1.push_str("ussama rah");
// // for word in st1.split_whitespace() {
// //     println!("{}", word);
// // }
// let st2 = st1.replace("sama", "samasama");
// println!("{}", st2);
// let st3 = String::from("h e l l o");
// let mut v1: Vec<char> = st3.chars().collect();
// v1.sort();
// v1.dedup();
// for c in v1 {
//     println!("{}", c);
// }
// let st4: &str = "random string";
// let mut st5 = st4.to_string();
// println!("{} {}", st4, st5);
// let byte_arr: &[u8] = st5.as_bytes();
// let st6 = &st5[0..5];
// println!("{} {}", st6, st6.len());
// st5.clear();
// let st6 = String::from("Just Some");
// let st7 = String::from(" words");
// let st8 = st6 + &st7;
// for c in st8.bytes() {
//     println!("{}", c);
// }
// let x: i8 = 127;
// let y: i8 = 127;
// let z: i16 = (x as i16) + (y as i16);
// println!("{}", z);
// }

// enum Day {
//     Mon,
//     Tue,
//     Wed,
//     Thu,
//     Fri,
//     Sat,
//     Sun,
// }

// impl Day {
//     fn is_weekend(&self) -> bool {
//         match self {
//             Day::Sat | Day::Sun => true,
//             _ => false,
//         }
//     }
// }

// fn main() {
//     // let x: u64 = 10;
//     // let x = String::from("hello");
//     // let x = Day::Mon;
//     // match x.is_weekend() {
//     //     true => print!("true\n"),
//     //     false => print!("false\n"),
//     // }
//     // let x = "💖";
//     // println!("{}", x);
//     let v: Vec<i32> = Vec::new();
//     let mut v2: Vec<i32> = vec![1, 2, 3];
//     match v2.get(4) {
//         Some(sec) => println!("sec == {}", sec),
//         None => println!("out of bound"),
//     }
//     for i in &mut v2 {
//         *i *= 2;
//     }
//     for i in &v2 {
//         println!("{}", i);
//     }
//     println!("pop : {:?}", v2.pop());
// }

// use std::ops::Add;

// fn pow<T: Add<Output = T> + Copy>(x: T) -> T {
//     return x + x;
// }

// fn main() {
//     let x = pow(5);
//     let f = pow(7.15);
//     println!("{} {}", x, f);
// }
// ownership
fn ret_str(s: String) -> String {
    // s
    String::from("hello")
}

fn main() {
    let s1 = String::from("test");
    // let s2 = s1;
    let s2 = ret_str(s1.clone());
    println!("{}", s2);
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("didnt receive input");
    // io::stdin().read_line(&mut name).unwrap();
}
