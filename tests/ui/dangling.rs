fn bad_ref<'a>() -> &'a str {
    let s = String::from("temp");
    &s
}

fn main() {
    let _ = bad_ref();
}
