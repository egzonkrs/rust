fn main() {
    let mut x = 4;
    let x = 5;
    let x = 6;
    println!("X is : {}", x);

    // Data types:
    let x = -123;
    // i8 - Range: -2^7 - 2^7 - 1
    // i16
    // i32 - default
    // i64
    // i128

    // Unassigned Integer - You can't use negative values only positives;
    let a: u32 = 123;
    // u8 - Range: 2^8 - 1
    // u16
    // u32 - default
    // u64
    // u128

    // Floating Points
    let f: f32 = 10.1543;

    // Booleans
    let tf: bool = true; // or just 1 - 0

    // Compound types
    let mut tuple: (i32, bool, char) = (1, false, '_');

    tuple = (10, true, 's');

    println!("{}", tuple.0);

    // Arrays
    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    arr[4] = 99;
    println!("{}", arr[4]);
}
