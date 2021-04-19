# Hello world

```rs
fn main() { // Rust programs need a main() function
    println!("Hello world!"); // println! is a macro. Macros end with !

    let name = "Jack";
    println!("My name is", name); // Concat in println! using commas
}
```

## To run
```sh
# 1. compile
rustc hello-world.rs

# 2. Run binary
./hello-world
```

# Print formatting

Rust provides various printing macros
1. `format!`: write formatted text to a string
2. `print!`: Similar to format but text is printed to console(`io::stdout`)
3. `println!`: Similar to `print!` but newline is appended
4. `eprint!` and `eprintln!`: Print to `io::stderr`. Used to display errors.

```rs
println!("I like {} and dogs", "cats");

println("I like {0} and {1}. I have two {0}", "cats", "dogs);
```

Formatting is done through markers implemented using traits:
1. `fmt::Display`: For `{}` marker or regular printing. It doesn't support complex types.
2. `fmt::Debug`: For `{:?}` marker or debug printing. Use `{:#?}` to pretty print. It lets us print custom types provided they derive from `fmt::Debug` using `#[derive(Debug)]`.

```rs
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
```

**Output:**

```
Printing name
Debug output: Person { name: "Peter", age: 10 }
Pretty printed debug output: Person {
    name: "Peter",
    age: 10,
}
The boy's name is "Peter"
```

## Custom formatting for derived types
`fmt::Debug` is not clean. But we can create custom formatters by implementing `fmt::Display`

```rs
use std::fmt;

#[derive(Debug)] // For debug formatting, unnecessary for custom formatting
struct Point2D {
    x: u8,
    y: u8
}

impl fmt::Display for Point2D {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point ({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point2D { x: 1, y: 5 };
    println!("{}", point); // custom formatting
    println!("{:?}", point); // debug formatting
}
```

**Output:**
```
Point (1, 5)
Point2D { x: 1, y: 5 }
```
