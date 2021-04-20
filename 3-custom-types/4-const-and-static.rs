// let age = 10; // ERROR- can't declare variables globally
const AGE_LIMIT: u32 = 20;
static MINIMUM_AGE: u32 = 5;

fn main() {
    println!("Age range: {}-{}", MINIMUM_AGE, AGE_LIMIT);
}
