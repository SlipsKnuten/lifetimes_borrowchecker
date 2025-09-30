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
    use std::{fs, path::PathBuf};

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

    // This test will FAIL at runtime (red in `cargo test`) and demonstrates
    // a borrow-rule violation enforced by RefCell's runtime checking.
    #[test]
    fn failing_runtime_borrow_violation() {
        let cell = RefCell::new(String::from("boom"));
        let _r = cell.borrow();
        let _w = cell.borrow_mut(); // panics -> this test FAILS
    }

    // Compile-fail tests (real borrow-checker/lifetime errors) using trybuild.
    // We keep everything in main.rs by writing the example sources to temp files first.
    #[test]
    fn compile_fail_lifetime_and_borrow_checker() {
        let tmp = tempfile::tempdir().unwrap();

        let mut write = |name: &str, src: &str| -> PathBuf {
            let path = tmp.path().join(name);
            fs::write(&path, src).unwrap();
            path
        };

        // Dangling reference: returning ref to local
        let p1 = write("dangling.rs", r#"
            fn bad_ref<'a>() -> &'a str {
                let s = String::from("temp");
                &s
            }
            fn main() {}
        "#);

        // Immutable + mutable overlap
        let p2 = write("overlap.rs", r#"
            fn main() {
                let mut s = String::from("hi");
                let r = &s;
                let m = &mut s;
                println!("{r}");
                m.push('!');
            }
        "#);

        // Two mutable borrows at once
        let p3 = write("double_mut.rs", r#"
            fn main() {
                let mut s = String::from("hi");
                let m1 = &mut s;
                let m2 = &mut s;
                m1.push('!');
                m2.push('?');
            }
        "#);

        // Non-'static capture in thread
        let p4 = write("non_static_thread.rs", r#"
            use std::thread;
            fn main() {
                let s = String::from("outer");
                thread::spawn(|| {
                    println!("{}", s);
                });
            }
        "#);

        // Rc across threads (not Send)
        let p5 = write("rc_across_threads.rs", r#"
            use std::{rc::Rc, thread};
            fn main() {
                let r = Rc::new(5);
                thread::spawn(move || {
                    let _ = r.clone();
                });
            }
        "#);

        let t = trybuild::TestCases::new();
        t.compile_fail(p1.to_str().unwrap());
        t.compile_fail(p2.to_str().unwrap());
        t.compile_fail(p3.to_str().unwrap());
        t.compile_fail(p4.to_str().unwrap());
        t.compile_fail(p5.to_str().unwrap());
    }
}
