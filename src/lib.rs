use std::{collections::HashSet, u128};
extern crate time;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use time::Instant;

// !!!cmk run parallel
// !!!cmk give sign that it is running
// !!!return the results equations
// !!!return the running time
// !!!return on equation at a time
// !!!release
// !!!small

use wasm_bindgen::prelude::*;

// See https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
// See https://rustwasm.github.io/book/game-of-life/code-size.html#shrinking-wasm-size
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn search(end: usize) -> String {
    // table.len < ((end-1)<sup>5</sup>*4)^(1/5) = (end-1)*4^(1/5) < (end-1)*1.32
    let table_len = (1.32 * (end - 1) as f64) as usize;
    let mut fifth: Vec<u128> = vec![];
    let mut valueset: HashSet<u128> = HashSet::new();
    for i in 0..table_len {
        let value = (i as u128).pow(5);
        valueset.insert(value);
        fifth.push(value);
    }

    let count: String = (4..end)
        //.into_par_iter()
        .map(|n4| {
            let mut subcount: Vec<String> = vec![];
            let mut sum = fifth[n4];

            for n3 in 3..n4 {
                let fn3 = fifth[n3];
                sum += fn3;

                for n2 in 2..n3 {
                    let fn2 = fifth[n2];
                    sum += fn2;

                    for n1 in 1..n2 {
                        let fn1 = fifth[n1];
                        sum += fn1;

                        if valueset.contains(&sum) {
                            let by5: u128 = ((sum as f64).powf(0.2f64) + 0.5f64) as u128;
                            let found = format!("<p>{n1}<sup>5</sup>+{n2}<sup>5</sup>+{n3}<sup>5</sup>+{n4}<sup>5</sup>={by5}<sup>5</sup></p>");

                            // println!("{:?} seconds for whatever you did.", start.elapsed());
                            subcount.push(found);
                            // return;
                        }
                        sum -= fn1
                    }
                    sum -= fn2;
                }
                sum -= fn3
            }
            subcount
        })
        .flatten()
        .collect::<Vec<String>>().join("");

    // println!("{} found in {:?} seconds.", count, start.elapsed());
    return format!("{:#?}", count);

    //return format!("{}", end + 2);
}

// pub fn searchx(end: usize) -> String {
//     let start = Instant::now();

//     // table.len < ((end-1)<sup>5</sup>*4)^(1/5) = (end-1)*4^(1/5) < (end-1)*1.32
//     let table_len = (1.32 * (end - 1) as f64) as usize;
//     let mut fifth: Vec<u128> = vec![];
//     let mut valueset: HashSet<u128> = HashSet::new();
//     for i in 0..table_len {
//         let value = (i as u128).pow(5);
//         valueset.insert(value);
//         fifth.push(value);
//     }
//     println!(
//         "Table of size {} created in {:?}",
//         table_len,
//         start.elapsed()
//     );

//     let count: usize = (4..end)
//         .into_par_iter()
//         .map(|n4| {
//             let mut subcount = 0;
//             let mut sum = fifth[n4];

//             for n3 in 3..n4 {
//                 let fn3 = fifth[n3];
//                 sum += fn3;

//                 for n2 in 2..n3 {
//                     let fn2 = fifth[n2];
//                     sum += fn2;

//                     for n1 in 1..n2 {
//                         let fn1 = fifth[n1];
//                         sum += fn1;

//                         if valueset.contains(&sum) {
//                             let by5: u128 = ((sum as f64).powf(0.2f64) + 0.5f64) as u128;
//                             println!("{}<sup>5</sup>+{}<sup>5</sup>+{}<sup>5</sup>+{}<sup>5</sup>={}<sup>5</sup>", n4, n3, n2, n1, by5);

//                             println!("{:?} seconds for whatever you did.", start.elapsed());
//                             subcount += 1;
//                             // return;
//                         }
//                         sum -= fn1
//                     }
//                     sum -= fn2;
//                 }
//                 sum -= fn3
//             }
//             subcount
//         })
//         .sum();

//     println!("{} found in {:?} seconds.", count, start.elapsed());
//     return format!("{}", count);
// }
