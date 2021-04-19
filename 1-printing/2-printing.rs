#[derive(Debug)]
struct Person<'a> { // Unprintable without deriving fmt::Debug
    name: &'a str,
    age: u8
}

fn main() {
    println!("{}", "Printing name");

    let name = "Peter";
    let age = 10;

    let peter = Person { name, age };

    // println!("{}", peter); // error, unsupported by fmt::Display
    println!("Debug output: {:?}", peter);
    println!("Pretty printed debug output: {:#?}", peter);

    // debug printing shows extra quotes "" for strings
    // This doesn't happen for regular display
    println!("The boy's name is {:?}", name);
}
