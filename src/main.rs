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

    if x.is_some() {
        println!("{:?}", x.unwrap());
    }

    let y: Option2<i32> = Option2::None;

    if !y.is_some() {
        println!("y is none");
    }

    let a = vec![];
    let mut b = a;

    b.push(1);

    println!("{:?}", a);
}
