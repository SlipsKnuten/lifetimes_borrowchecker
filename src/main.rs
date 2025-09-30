use std::cell::RefCell;
use std::thread;
use std::time::Duration;

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

pub struct Wrapper<'a> {
    pub inner: &'a str,
}

pub fn borrow_scopes_ok() -> String {
    let mut s = String::from("hello");
    let a = &s;
    let b = &s;
    let msg = format!("reads: {a} & {b}");
    let m = &mut s;
    m.push_str(", world");
    msg
}

pub fn refcell_ok() -> String {
    let cell = RefCell::new(String::from("hi"));
    {
        let mut w = cell.borrow_mut();
        w.push_str(" there");
    }
    cell.borrow().clone()
}

pub fn thread_move_owned_demo() -> String {
    let owned = String::from("moved into thread safely");
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        tx.send(owned).unwrap();
    });
    let out = rx.recv().unwrap();
    handle.join().unwrap();
    out
}

fn main() {
    println!("-- Borrow & Lifetime Lab --");
    println!("longest(\"short\", \"a little longer\") => {}",
        longest("short", "a little longer"));
    let s = String::from("wrapped");
    let w = Wrapper { inner: &s };
    println!("Wrapper.inner => {}", w.inner);
    let msg = borrow_scopes_ok();
    println!("borrow_scopes_ok() => {}", msg);
    let ok = refcell_ok();
    println!("refcell_ok() => {}", ok);
    let threaded = thread_move_owned_demo();
    println!("thread_move_owned_demo() => {}", threaded);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_longest() {
        let a = String::from("short");
        let b = String::from("a little longer");
        assert_eq!(longest(&a, &b), "a little longer");
    }

    #[test]
    fn t_wrapper_lifetime() {
        let s = String::from("wrapped");
        let w = Wrapper { inner: &s };
        assert_eq!(w.inner, "wrapped");
    }

    #[test]
    fn t_borrow_scopes_ok() {
        let msg = borrow_scopes_ok();
        assert!(msg.contains("reads: hello & hello"));
    }

    #[test]
    fn t_refcell_ok() {
        assert_eq!(refcell_ok(), "hi there");
    }

    #[test]
    fn t_thread_move_owned_demo() {
        assert_eq!(thread_move_owned_demo(), "moved into thread safely");
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn refcell_overlapping_borrows_panics() {
        let cell = RefCell::new(String::from("boom"));
        let _r = cell.borrow();
        let _w = cell.borrow_mut();
    }

    #[test]
    #[should_panic]
    fn refcell_two_mut_panics() {
        let cell = RefCell::new(0);
        let _m1 = cell.borrow_mut();
        let _m2 = cell.borrow_mut();
    }

    #[test]
    fn t_intentional_failure() {
        assert_eq!(1, 2);
    }
}
