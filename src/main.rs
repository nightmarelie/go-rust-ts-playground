fn main() {
    let mut a: Vec<i32> = vec![];
    let b = &mut a;

    b.push(1);

    println!("{:?}", b);
    println!("{:?}", a);
}
