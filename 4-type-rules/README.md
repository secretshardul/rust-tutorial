# Type casting
Rust has no implicit casts. Explicit casting must be done using `as` keyword.

## Notes
- When casting to unsigned numbers, MSBs equal to the type size are kept. The rest are truncated due to overflow.

```rs
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
```

# Type inference
The compiler can look at usage of a variable and infer its type.

```rs
fn main() {
    let my_num = 5;
    let my_vector = Vec::new(); // Type not specified

    my_vector.push(my_num); // u32 type dynamically inferred based on type of my_num
}
```

# Type aliases
We can use aliases for types to reduce boilerplate. Aliases must have CamelCase names. They don't provide extra type safety since they're only aliases.

```rs
type NanoSecond = u64;

fn main() {
    let time: NanoSecond = 24356;
    println!("Time: {}", time); // Can use fmt::Display directly since it's only an alias
}
```

# Type conversions
Type conversions are performed using `From` and `To` traits. They're already implemented for some generic types like `String`.

```

```