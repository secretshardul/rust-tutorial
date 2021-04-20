# Rust custom types

## Structs
Rust supports 3 types of structs:
1. **C style structs** having variables inside
2. **Tuple structs**: They are named tuples
3. **Unit structs**: They are fieldless. Useful for generics.

```rs
/* 1. C style structs */
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Line {
    p1: Point, // Struct Point is used as a type inside another struct
    p2: Point
}

/* 2. Tuple struct */

// Tuples can be debug printed directly, but not struct tuples. Need to derive fmt::Debug
#[derive(Debug)]
struct Grades(f32, f32, f32);

/* 3. Unit struct */
#[derive(Debug)]
struct Unit;

fn main() {
    let p1 = Point { x: 1.0, y: 3.0 };
    let p2 = Point { x: 3.2, y: -5.1 };
    let line = Line { p1, p2 };

    println!("Line: {:?}", line);

    let grades = Grades(91.0, 86.46, 97.3);
    println!("Grades: {:?}", grades);

    let unit = Unit;
    println!("Unit struct: {:?}", unit);
}
```

## Enums

Enum keyword allows the creation of a type which may be one of a few variants. Variants are actually struct definitions.

```rs
enum WebEvent {
    PageLoad, // Unit like
    KeyPress(char), // Tuple
    Click { x: i32, y: i32 } // Struct
}

fn inspect(event: WebEvent) {
    // Matching with Enum
    match event {
        WebEvent::PageLoad => println!("Page loading"),
        WebEvent::KeyPress(c) => println!("Pressed key {}", c),
        WebEvent::Click { x, y } => println!("Clicked at {}, {}", x, y)
    }
}

fn main() {
    // Create structs using enum variants
    let page_load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('c');
    let click = WebEvent::Click { x: 32, y: 6 };

    inspect(pageLoad);
    inspect(press);
    inspect(click);
}
```

### Notes
- Enum names are in Camel Case.
- Double colons `::` are used instead of dot `.` to access enum variants.
    - **Double colons ::**: To access items inside modules
    - **dot .**: To access fields and methods inside structs.

- We can use aliases for long enums. This is generally done in implementations through `Self`

```rs
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers; // Alias

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> {
        match (self) {
            Self::Add => x + y, // Use Self instead of enum name
            Self::Subtract => x - y
        }
    }
}
```

- To access enum variants directly without manual scoping, the `use` keyword is used. Syntax: `use crate::EnumName::{ variantToImport }` or `use crate::EnumName::*` to import all.

```rs
#[derive(Debug)]
enum Status {
    Rich,
    Poor
}

#[derive(Debug)]
enum Work {
    Civilian,
    Soldier
}

fn main() {
    // Make variants available without manual scoping
    use crate::Status::{ Rich, Poor };
    use crate::Work::*; // Wildcard- import all variants

    let my_status = Rich; // No need of using Status::Rich now
    let work = Soldier;

    println!("Status: {:?}, occupation: {:?}", my_status, work);
}
```

- We can define C-style enums with explicit discriminators

```rs
enum RustLikeEnum {
    First, // Implicit discriminator
    Second
}

enum CppLikeEnum {
    Red = 0x1345, // Explicit discriminator
    Blue = 0x5463
}
```

## Constants- const and static
- Constant values can't be changed.
- Their type must be explicitly specified.
- They can be declared in any scope including global. Variables can't be declared globally.
- Contants are of two types:
    1. `const`: They represent values, not memory locations in the case of `let`.
    2. `static`: Refer to a memory location. All references to static point to the same location. They have a `static` lifetime, which outlives all other lifetimes in a rust program.


```rs
// let age = 10; // ERROR- can't declare variables globally
const AGE_LIMIT: u32 = 20;
static MINIMUM_AGE: u32 = 5;

fn main() {
    println!("Age range: {}-{}", MINIMUM_AGE, AGE_LIMIT);
}
```
