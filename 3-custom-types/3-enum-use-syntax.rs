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
