fn main() {
    let my_float = 65.67;
    let my_int = my_float as u8;
    let my_char = my_int as char; // Only u8 can be cast to char, not other numbers or floats

    println!("Converting {} -> {} -> {}", my_float, my_int, my_char);


    /* Loss of precision due to underflow */
    let my_u16 = 1000;
    let converted_u8 = my_u16 as u8; // First 8 MSB are kept, rest get truncated.
    println!("u16 {} -> u8 {}", my_u16, converted_u8); // u16 1000 -> u8 232
}