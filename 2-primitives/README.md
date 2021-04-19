# Rust primitives

## Primitive types

1. **Scalar types / Literals**
    1. Signed integers: i8, i16, i32 (default), i64, i128, isize(pointer size)
    2. Unsigned integers: u8, u16, u32, u64, u128, usize(pointer size)
    3. Float: f32, f64 (default)
    4. `char`
    5. `bool`: `true` or `false`
    6. Empty tuple `()`

2. **Compound types**
    1. Arrays: Eg `[1, 2, 3]`
    2. Tuples: Eg. `(1, false)`. A tuple can contain  elements of different types and can be of arbitary length. [Look at tuple reversing example](./2-tuples.rs)

## Type rules

- Type can be inferred at compile time. Look at `inferred_num` below.
- Types can be explicitly set using
    1. Regular annotations

        ```rs
        let small_num: u8 = 2;
        ```

    2. Suffix annotations

        ```rs
        let big_num = 10i64; // NOTE i64 at end
        ```
- Types can be overridden through shadowing, i.e. declaring a new variable with the same name. Shadowing is restricted to bracket scope.

```rs
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
```

## Representing number types
1. Number notations:
    - Binary: `0b`
    - Octal: `0o`
    - Hex: `0x`

2. Underscores to improve readibility

```rs
let a = 1000;
let b = 1_000; // same as a
```

***

## Arrays vs Slices

### Arrays
Arrays have a fix length, decided at compile time. Length is part of their signature `[T; length]`

```rs
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
```

### Slices
Slices are like arrays but with dynamic lengths. A slice object has two parts:
1. Pointer to the first element
2. Length: It is of type `usize`. It's memory size depends on system architecture.

```rs
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
```
