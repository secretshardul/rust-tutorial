fn main() {
    let my_num = 5;
    let my_vector = Vec::new(); // Type not specified

    my_vector.push(my_num); // u32 type dynamically inferred based on type of my_num
}