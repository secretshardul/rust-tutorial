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