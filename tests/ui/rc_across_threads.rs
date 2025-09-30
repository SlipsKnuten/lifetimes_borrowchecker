use std::{rc::Rc, thread};

fn main() {
    let r = Rc::new(5);
    thread::spawn(move || {
        let _ = r.clone();
    });
}
