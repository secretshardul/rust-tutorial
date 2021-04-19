fn main() {
    let count = 10; // default i32
    let pi = 3.14; // default f64

    let mut inferred_num = 4; // Inferred as i64 from next line
    inferred_num = 4294967296i64; // NOTE i64 at end

    // Type annotations to explicitly set type
    let small_num: u8 = 2; // Regular annotation
    let big_num = 10i64; // Suffix annotation

    // Shadowing allows us to change variable type in scope
    let mut num_or_bool = 4;

    // num_or_bool = true; // Throws error

    let num_or_bool = true; // Shadowed
}
