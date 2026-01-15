#![allow(unused)]

fn main() {
    // loop
    let mut i = 0;
    loop {
        println!("Loop {i}");
        if i == 5 {
            break;
        }
        i += 1;
    }
    // while
    let mut i = 0;
    while i <= 3 {
        println!("while {i}");
        i += 1;
    }
    // for loop
    for i in 0..5 {
        println!("for {i}");
    }

    for i in 0..=5 {
        println!("for {i}");
    }
    // for loop array
    let arr = [1, 2, 3];
    for a in arr {
        println!("array {a}");
    }
    // usize and range
    let n = arr.len(); // this is type usize
    for i in 0..n {
        println!("array {}", arr[i]);
    }
    // for loop vector
    let v = vec![1, 2, 3];

    // iter (iterator) usable with hashmaps, arrays and vectors.

    for x in v.iter() {
        println!("vec {x}");
    }

    for x in v.iter() {
        println!("vec {x}");
    }

    // Return value
    let mut i = 0;
    let z = loop {
        println!("Loop {i}");
        if i == 3 {
            break 99; // break and return 99 to z
        }
        i += 1;
    };
    println!("reutrn loop: {z}");

    // labels
    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            println!("{i}, {j}");
            if i == 1 && j == 2 {
                break 'outer;
            }
        }
    }
}
