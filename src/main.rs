#[derive(Debug)]
enum RustEnum {
    A = 1,
    B = 2,
    C = 3
}

fn main() {
    println!("{:?}", RustEnum::A);
    println!("{:?}", RustEnum::B);
    println!("{:?}", RustEnum::C);
}
