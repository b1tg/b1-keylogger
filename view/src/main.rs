// snippet of code @ 2020-03-13 09:55:32

// === Rust Playground ===
// This snippet is in: ~/.emacs.d/rust-playground/at-2020-03-13-095531/

// Execute the snippet: C-c C-c
// Delete the snippet completely: C-c k
// Toggle between main.rs and Cargo.toml: C-c b
extern crate rayon;
extern crate chrono;
extern crate dashmap;
extern crate colored;
use colored::*;
use ansi_term::Colour::Red;
use ansi_term::Colour::Fixed;
use ansi_term::Colour::RGB;
use rayon::prelude::*;
use chrono::prelude::*;
use dashmap::DashMap;
use std::fs;
use std::collections::{HashMap};

// fn word_counts(s: &str) -> Vec<(&str, usize)> {
//     let map = DashMap::new();
//     s.par_split_whitespace().for_each(|word| {
//         *map.entry(word).or_insert(0) += 1;
//     });
//     let mut vec = map.iter().collect();
//    // vec.par_sort_by_key(|(k, v)| v);
//     vec
// }

// fn main() {
//     let cc = word_counts("abc ab cd ab");
//     println!("{:?}", cc);
// }

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}


fn main() {
    let enabled = ansi_term::enable_ansi_support();
    let content: String = fs::read_to_string("./key.log").expect("Unable to read file");
    let lines = content.lines();

    //let db: Vec<(DateTime<Local>, i32)> = Vec::new();
    let mut map = HashMap::new();
    for line in lines {
        let ts = line.split(",").nth(0).unwrap().trim();

        let tsf = ts.parse::<DateTime<Local>>().unwrap();
        let tsk = (tsf.ordinal(),tsf.hour(), tsf.minute());
        //println!("ts: {:?} {:?} {:?}", tsf.ordinal(),tsf.hour(), tsf.minute());
        //println!("tsk: {:?}", tsk);
       // if tsk.0 == 72 {
        let entry = map.entry(tsk).or_insert(0);
        *entry += 1;
        //}
    }
    //println!("map: {:?}", map);
   // let mut pair = map.iter().collect::<Vec<(_, _)>>();
    // let mut filter_pair = pair
    //     .filter(|x| {
    //         let i = (x.0).0;
    //         println!("i: {}", i);
    //         return i == 72;
    //     })
   // println!("pair: {:?}", pair);
    //pair.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    for d in 71..74 {
        println!("Day {}:", d);
        for i in 0..24 {
            print!("{:2} ", i);
            for j in 0..60 {
                if map.contains_key(&(d,i,j)) {
                    let grey = 51; // 51-46
                    let vv = map.get(&(d,i,j)).unwrap();
                    let  vvv = vv.clone() / 75;
                    //let color_list = [RGB(0xc6,0x14,0x8b),RGB(0,255,215),RGB(0,255,175),
                     //                 RGB(0,255,135),RGB(0,255,95), RGB(0,255,0) ];
                    let color_list = [RGB(0xc6,0xe4,0x8b),RGB(0x7b,0xc9,0x6f),RGB(0x23,0x9a,0x3b),
                                      RGB(0x19,0x61,0x27)];
                    // if vvv < 46 {
                    //     vvv = 46
                    // }
                    // if vvv > 51 {
                    //     vvv =
                    // }
//                    print!("{}", color_list[vvv].paint(vvv.to_string()));
                    print!("{}", color_list[vvv].paint("+"));
                   // print!("{}", "+".on_white().green());
                } else {
                   // print!("{}", "-".on_white().white());
                    print!("-");
                }
            }
            print!("\n");
        }
        println!("");
    }

    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
    println!("{} {} {} {}", "black".black(),"red".red(), "green".green(), "yellow".yellow());
println!("{} {} {} {}", "blue".blue(), "magenta".purple(), "cyan".cyan(), "white".white());
}

// fn main() {
// //    let a =    sum_of_squares(&[1,2,3,4,5]);
//   //  println!("sum: {}", a);



//     for i in 0..24 {
//         print!("{:2} ", i);
//         for j in 0..60 {
//             if j%7 == 0 {
//                 print!("-");
//             } else {
//                 print!("+");
//             }
//         }
//         print!("\n");
//     }
// }
