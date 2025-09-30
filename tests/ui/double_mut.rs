fn main() {
    let mut s = String::from("hi");
    let m1 = &mut s;
    let m2 = &mut s;
    m1.push('!');
    m2.push('?');
}
