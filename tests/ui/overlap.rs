fn main() {
    let mut s = String::from("hi");
    let r = &s;
    let m = &mut s;
    println!("{r}");
    m.push('!');
}
