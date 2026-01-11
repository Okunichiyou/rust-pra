fn main() {
    let mut s = "Hello".to_string();

    let s_ref = &mut s;
    clear_s(s_ref);
    println!("s= {}", s);

    let s_ref2 = &mut s;
    clear_s(s_ref2);
    println!("s= {}", s);
}

fn clear_s(s: &mut String) {
    s.clear();
}
