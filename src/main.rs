fn main() {
    let mut s = "hello".to_string();
    println!("s= {}", s);
    clear_s(&mut s);
    println!("s= {}", s);
}

fn clear_s(s: &mut String) {
    s.clear();
}
