fn main() {
    let s = "hello".to_string();
    print_some(s);
    print_some(s);
}

fn print_some(s: String) {
    println!("{}", s)
}
