type NanoSecond = u64;

fn main() {
    let time: NanoSecond = 24356;
    println!("Time: {}", time); // Can use fmt::Display directly since it's only an alias
}