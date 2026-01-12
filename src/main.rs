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

fn longest<'a>(x: &'a str, y: &str) -> String {
    let result = String::from("xxx yyy zzz");
    result
}

// resultのライフタイムは関数内で終了するので、resultへの参照を返せない
// result.clone().as_str()とかにしても同じ
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     let result = String::from("xxx yyy zzz");
//     result.as_str()
// }
