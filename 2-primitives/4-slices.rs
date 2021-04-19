fn get_slice_length(slice: &[u8]) -> usize {
    slice.len() // Find length
}
fn main() {
    let my_slice = &[1, 2, 3];

    println!("Slice {:?}", my_slice);
    println!("First slice element: {}", my_slice[0]); // Indexing
    println!("Slice length: {:?}", get_slice_length(my_slice)); // Find length

    let arr = [6, 7, 8];

    let copied_slice = &arr; // Borrow array as slice
    println!("Copied slice {:?}", copied_slice);

    // Error- can't edit array when it is borrowed
    // arr = [9, 10, 11];
    // println!("Slice after array is modified: {:?}", copied_slice);
}