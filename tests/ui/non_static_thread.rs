use std::thread;

fn main() {
    let s = String::from("outer");
    thread::spawn(|| {
        println!("{s}");
    });
}
