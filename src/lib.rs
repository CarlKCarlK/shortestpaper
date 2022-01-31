use fxhash::FxHashSet;
use wasm_bindgen::prelude::*;

// See https://www.reddit.com/r/rust/comments/sf8mqp/recreating_worlds_shortest_math_paper_with_rust/
// See https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

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
    // table.len < ((end-1)^5*4)^(1/5) = (end-1)*4^(1/5) < (end-1)*1.32
    let table_len = (1.32 * (end - 1) as f64) as usize;
    let mut fifth: Vec<u128> = vec![];
    let mut valueset: FxHashSet<u128> = Default::default();
    for i in 0..table_len {
        let value = (i as u128).pow(5);
        fifth.push(value);
    }

    // precalculate a^5+b^5+c^5 values (time-memory tradeoff)
    for i in 0..table_len {
        let sum = fifth[i];
        for sub in &fifth[..i] {
            valueset.insert(sum - sub);
        }
    }

    let count: String = (1..end)
        //.into_par_iter()
        .map(|n1| {
            let mut subcount: Vec<String> = vec![];
                let sum = fifth[n1];

                for n2 in 2..n1 {
                    let sum = sum + fifth[n2];

                    for n3 in 3..n2 {
                        let sum = sum + fifth[n3];

                        if valueset.contains(&sum) {
                            // this loop is O(n log n) but only runs once
                            for n4 in 4..n3 {
                                let sum = sum + fifth[n4];
                                if let Ok(n5) = fifth.binary_search(&sum) {
                                    let found = format!("<p>{n4}<sup>5</sup>+{n3}<sup>5</sup>+{n2}<sup>5</sup>+{n1}<sup>5</sup>={n5}<sup>5</sup></p>");
                                    subcount.push(found);
                                }
                            }
                    }
                }
            }
            subcount
        })
        .flatten()
        .collect::<Vec<String>>().join("");

    return count;
}
