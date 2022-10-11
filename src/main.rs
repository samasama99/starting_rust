#![allow(unused)]

// use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{f32::consts::PI, io, string};

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
// fn ret_str(s: String) -> String {
//     // s
//     String::from("hello")
// }

// fn main() {
//     let s1 = String::from("test");
//     // let s2 = s1;
//     let s2 = ret_str(s1.clone());
//     println!("{}", s2);
// }

// use std::collections::HashMap;

// fn main() {
//     let mut names = HashMap::new();
//     names.insert("sama", 23);
//     names.insert("test", 13);
//     names.insert("not test", 33);
//     for (k, v) in names.iter() {
//         println!("{} {}", k, v);
//     }
//     if names.contains_key(&"sama") {
//         println!("contained");
//     }
//     match names.get("test") {
//         Some(x) => println!("h age is {}", x),
//         None => println!("not here"),
//     }
//     match names.get("nah") {
//         Some(x) => println!("h age is {}", x),
//         None => println!("not here"),
//     }
// }

// use std::fmt;
// use std::fmt::*;

// #[derive(Debug)]
// struct Vector {
//     x: f64,
//     y: f64,
//     z: f64,
// }

// impl fmt::Display for Vector {
//     // This trait requires `fmt` with this exact signature.
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         write!(f, "my print : {},{},{}\n", self.x, self.y, self.z)
//     }
// }

// fn main() {
//     let v = Vector {
//         x: 1.2,
//         y: 0.5,
//         z: 0.0,
//     };
//     println!("{:#?}", v);
//     println!("{:?}", v);
//     println!("{}", v);
//     println!("{}", v.to_string());

//     struct Rec<T, U> {
//         len: T,
//         height: U,
//     }
//     let r = Rec {
//         len: 4,
//         height: 10.5,
//     };
// }

// fn main() {
//     trait Shape {
//         fn new(len: f32, width: f32) -> Self;
//         fn area(&self) -> f32;
//     }

//     struct Rec {
//         len: f32,
//         width: f32,
//     };

//     struct Circle {
//         len: f32,
//         width: f32,
//     };

//     impl Shape for Rec {
//         fn new(len: f32, width: f32) -> Rec {
//             Rec { len, width }
//         }

//         fn area(&self) -> f32 {
//             self.len * self.width
//         }
//     }

//     impl Shape for Circle {
//         fn new(len: f32, width: f32) -> Circle {
//             Circle { len, width }
//         }

//         fn area(&self) -> f32 {
//             (self.len / 2.0).powf(2.0) * PI
//         }
//     }

//     let r: Rec = Shape::new(10.0, 10.0);
//     let c: Circle = Shape::new(10.0, 10.0);
//     println!("{} {}", r.area(), c.area());
// }

// modules ::

// mod restaurant;
// use crate::restaurant::order_food;

// fn main() {
//     order_food();
// }

// ERRORS

// fn main() {
//     // panic!("Terrible Error");

//     // let arr = [1, 2];
//     // arr[2];

//     let path = "lines.txt";
//     let output = File::create(path);
//     let mut output = match output {
//         Ok(file) => file,
//         Err(error) => panic!("Error :: creating file : {:?}", error),
//     };
//     writeln!(output, "Just testing !!").expect("failed to write to file");
//     writeln!(output, "another line !!").expect("failed to write to file");
//     writeln!(output, "Bye !!").expect("failed to write to file");

//     let input = File::open(path).unwrap();
//     let buffered = BufReader::new(input);

//     for line in buffered.lines() {
//         println!("{}", line.unwrap());
//     }

//     let output2 = match File::open("rand.txt") {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("rand.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("can't create file: {:?}", error),
//             },
//             _other_error => panic!("Problem opening file : {:?}", error),
//         },
//     };
// }

// iter ::

// fn main() {
// let mut arr = [1, 2, 3, 4];
// for val in arr.iter() {
//     println!("{}", val);
// }
// // arr.into_iter();
// let mut iter1 = arr.iter();
// println!("1st : {:?}", iter1.next());
// println!("2st : {:?}", iter1.next());
// println!("3st : {:?}", iter1.next());
// println!("4st : {:?}", iter1.next());
// println!("5st : {:?}", iter1.next());
// }

// colosure

// fn main() {
//     // let can_vote = |age: i32| age >= 18;
//     // println!("can vote {}", can_vote(13));
//     // println!("can vote {}", can_vote(23));

//     // let mut samp1 = 5;
//     // let print_var = || println!("samp1 = {}", samp1);
//     // print_var();
//     // samp1 = 10;
//     // let mut change_value = || samp1 += 1;
//     // change_value();
//     // println!("samp1 = {}", samp1);

//     fn use_func<T>(a: i32, b: i32, func: T) -> i32
//     where
//         T: Fn(i32, i32) -> i32,
//     {
//         func(a, b)
//     }

//     let sum = |a: i32, b: i32| a + b;
//     let prod = |a: i32, b: i32| a * b;
//     println!("{}", use_func(10, 10, sum));
//     println!("{}", use_func(10, 10, prod))
// }

// Box :: smart pointer

// fn main() {
//     // let b_int1 = Box::new(10);
//     // println!("{}", b_int1);

//     // struct TreeNode<T> {
//     //     pub left: TreeNode<T>,
//     //     pub right: TreeNode<T>,
//     //     pub key: T,
//     // }
//     struct TreeNode<T> {
//         pub left: Option<Box<TreeNode<T>>>,
//         pub right: Option<Box<TreeNode<T>>>,
//         pub key: T,
//     }
//     impl<T> TreeNode<T> {
//         pub fn new(key: T) -> Self {
//             TreeNode {
//                 left: None,
//                 right: None,
//                 key,
//             }
//         }
//         pub fn left(mut self, node: TreeNode<T>) -> Self {
//             self.left = Some(Box::new(node));
//             self
//         }
//         pub fn right(mut self, node: TreeNode<T>) -> Self {
//             self.right = Some(Box::new(node));
//             self
//         }
//     }

//     let node1 = TreeNode::new(1)
//         .left(TreeNode::new(2))
//         .right(TreeNode::new(3));
// }

use std::thread;
use std::time::Duration;

// fn main() {
//     let th = thread::spawn(|| {
//         for i in 1..25 {
//             println!("Spawned thread : {}", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     for i in 1..20 {
//         println!("Main thread : {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     th.join().unwrap();
// }

pub struct Bank {
    balance: f32,
}

// fn withdraw(the_bank: &mut Bank, amt: f32) {
//     the_bank.balance -= amt;
// }

// fn customer(the_bank: &mut Bank) {
//     withdraw(the_bank, 5.0);
// }

// fn main() {
//     let mut b = Bank { balance: 100.0 };
//     // withdraw(&mut b, 5.0);
//     // println!("Balance : {}", b.balance);
//     thread::spawn(|| customer(&mut b)).join().unwrap();
// }

fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
    let mut bank_ref = the_bank.lock().unwrap();
    if (bank_ref.balance < 5.00) {
        println!("Current balance : {} withdraw {}", bank_ref.balance, amt);
    } else {
        bank_ref.balance -= amt;
        println!("Current balance : {} withdraw {}", bank_ref.balance, amt);
    }
}

fn customer(the_bank: Arc<Mutex<Bank>>) {
    withdraw(&the_bank, 5.0);
}

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 100.0 }));
    let handles = (0..20).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref);
        })
    });
    for handle in handles {
        handle.join();
    }
    println!("Total {}", bank.lock().unwrap().balance);
    println!("done")
}
