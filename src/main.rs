struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // 僕をイシュマエルとお呼び。何年か前・・・
    let novel = String::from(".");
    //                                                  "'.'が見つかりませんでした"
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
