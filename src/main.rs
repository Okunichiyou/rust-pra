fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let v2 = [3, 4, 5];

    let p = pick2(&v1, &v2, 3);
    for ss in p.0 {
        println!("{}", ss);
    }

    for ss in p.1 {
        println!("{}", ss);
    }
}

fn pick1(x: &[i32], end: usize) -> &[i32] {
    &x[..end]
}

fn pick2(x: &[i32], y: &[i32], end: usize) -> (&[i32], &[i32]) {
    (&x[..end], &y[..end])
}
