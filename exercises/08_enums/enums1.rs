#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(u32, u32),
    Move(i32, i32),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize(800, 600));
    println!("{:?}", Message::Move(10, 20));
    println!("{:?}", Message::Echo(String::from("Hello")));
    println!("{:?}", Message::ChangeColor(255, 0, 128));
    println!("{:?}", Message::Quit);
}
