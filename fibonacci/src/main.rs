use std::collections::HashMap;

const FIB_ONE: u64 = 1;

// The standard recursive fibonacci
// Not terribly great for performance.
fn fib(n: u64) -> u64 {
    if n == 0 {
        FIB_ONE
    } else if n == 1 {
        FIB_ONE
    } else {
        fib(n - 1) + fib(n -2)
    }
}

// Rather than use a recursive call for every computation
// cache the results and look them up if possible for future calls.
// The best data structure for this is the HashMap, as we want
// efficient lookups by arbitrary keys.
fn fast_fib(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => FIB_ONE,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = fast_fib(n -1, map) + fast_fib(n -2, map);
                map.insert(n, val);
                return val;
            }
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    for i in 1..41 {
        println!("Result for {}: - {}", i, fast_fib(i, &mut map));
    }
}