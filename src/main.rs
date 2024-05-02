fn main() {
//     let mut a = vec![];
//     let b = &mut a;
//
//     b.push(1);
//
//     println!("{:?}", b);
//     println!("{:?}", a);

    let mut x = 5;
    let z = &mut x;

    *z = 7;
    println!("{:?}", z);
}
