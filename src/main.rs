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
    println!("longest = {}", longest("short", "a little longer"));
    let s = String::from("wrapped");
    let w = Wrapper { inner: &s };
    println!("Wrapper.inner = {}", w.inner);
    println!("borrow_scopes_ok = {}", borrow_scopes_ok());
    println!("refcell_ok = {}", refcell_ok());
    println!("thread_move_owned_demo = {}", thread_move_owned_demo());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passing_runtime_tests() {
        let a = String::from("short");
        let b = String::from("a little longer");
        assert_eq!(longest(&a, &b), "a little longer");

        let s = String::from("wrapped");
        let w = Wrapper { inner: &s };
        assert_eq!(w.inner, "wrapped");

        assert!(borrow_scopes_ok().contains("reads: hello & hello"));
        assert_eq!(refcell_ok(), "hi there");
        assert_eq!(thread_move_owned_demo(), "moved into thread safely");
    }

    #[test]
    fn failing_runtime_borrow_violation() {
        // Demonstrates runtime borrow-rule panic; this test is expected to FAIL.
        let cell = RefCell::new(String::from("boom"));
        let _r = cell.borrow();
        let _w = cell.borrow_mut();
    }

    // Compile-fail tests (real borrow-checker/lifetime errors) using trybuild.
    // Source snippets live under tests/ui with matching .stderr expectations.
    #[test]
    fn compile_fail_lifetime_and_borrow_checker() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/*.rs");
    }
}
