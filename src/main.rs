use std::{collections::HashSet, u128};
extern crate time;
use time::Instant;

fn main() {
    let start = Instant::now();

    let mut fifth: Vec<u128> = vec![];
    let mut valueset: HashSet<u128> = HashSet::new();

    for n4 in 4..1500 {
        for i in fifth.len()..4 * n4 - 3 {
            let value = (i as u128).pow(5);
            valueset.insert(value);
            fifth.push(value);
        }

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
                        println!("{}^5+{}^5+{}^5+{}^5={}^5", n4, n3, n2, n1, by5);

                        let end = Instant::now();
                        println!("{:?} seconds for whatever you did.", end - start);
                        // return;
                    }
                    sum -= fn1
                }
                sum -= fn2;
            }
            sum -= fn3
        }
    }
}
