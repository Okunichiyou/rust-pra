fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }

    // resultのライフタイムはここまで存在しているが、string2のライフタイムがすでに終わっている
    // resultを使うにはstring2のライフタイムがここまで存在する必要があるけど、存在していないのでコンパイルエラーになる
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
