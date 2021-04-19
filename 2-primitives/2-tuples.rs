fn reverse(pair: (u32, bool)) -> (bool, u32) {
    let (number, bool_value) = pair;

    return (bool_value, number);
}

fn main() {
    let pair =  (9, false);
    println!("Original pair: {:?}", pair);

    println!("Reversed: {:?}", reverse(pair));
}