#![allow(unused)]

// Tipos de datos compuestos
// - tuple
// - array

fn main() {
    // Array - tamaño fijo y conocido en compile time
    let arr: [u32; 3] = [1, 2, 3]; // [type; size]
    println!("arr: {} {} {}", arr[0], arr[1], arr[2]);

    let mut marr: [u32; 3] = [1, 2, 3];
    marr[1] = 9;

    let farr: [u32; 5] = [0; 5]; // {0, 0, 0, 0, 0}
    println!("farr: {:?}", farr);

    // Slice - tamaño conocido en runtime
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 5, -5, 5];

    // First 3 elements
    let s = &nums[0..3]; // Take nums from 0 to 3
    println!("Free three elements: {:?}", s);

    // Last 3 elements
    let s = &nums[7..]; // Take nums from 7 to end
    println!("Last three elements: {:?}", s);

    // All elements
    let s = &nums[..]; // Take all nums
    println!("Free three elements: {:?}", s);
}
