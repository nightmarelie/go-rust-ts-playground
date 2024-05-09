#[derive(Debug)]
enum Option2<T> {
    None,
    Some(T),
}

impl <T> Option2<T> {
    fn unwrap(self) -> T {
        match self {
            Option2::None => panic!("called `Option::unwrap()` on a `None` value"),
            Option2::Some(value) => value,
        }
    }
}

fn main() {
    let x = Option2::Some(10);
    println!("{:?}", x.unwrap());

    let y: Option2<i32> = Option2::None;
    println!("{:?}", y.unwrap());
}
