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
