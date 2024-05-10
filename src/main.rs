#[derive(Debug)]
enum Option2<T> {
    None,
    Some(T),
}

impl <T> Option2<T> {
    fn unwrap(&self) -> &T {
        match self {
            Option2::None => panic!("called `Option::unwrap()` on a `None` value"),
            Option2::Some(value) => value,
        }
    }

    fn is_some(&self) -> bool {
        match self {
            Option2::None => false,
            Option2::Some(_) => true,
        }
    }
}

fn main() {
    let x = Option2::Some(10);
    println!("{:?}", x.unwrap());
    println!("{:?}", x.is_some());

    let y: Option2<i32> = Option2::None;
    println!("{:?}", y.unwrap());
    println!("{:?}", y.is_some());
}
