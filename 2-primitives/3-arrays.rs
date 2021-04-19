use std::mem;

fn main() {
    let arr = [1, 2, 3, 5];
    println!("Array:  {:?}", arr);

    let explicit_arr: [u8; 4] = [1, 2, 3, 4];
    println!("Array with explicit type: {:?}", explicit_arr);

    // Index elements using [] brackets
    println!("Item at index 3: {}", arr[3]);

    // Find array length
    println!("First array length: {}", arr.len());

    // Find array size
    println!("First array size: {}", mem::size_of_val(&arr));
}
