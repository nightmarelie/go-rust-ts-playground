#[derive(Debug)]
enum RSEnum {
    Foo(i32),
    Bar(String),
    Baz(Vec<String>),
}

fn main() {
    let foo = RSEnum::Foo(42);
    let bar = RSEnum::Bar("Hello".to_string());
    let baz = RSEnum::Baz(vec!["a".to_string(), "b".to_string()]);

    println!("{:?}", foo);
    println!("{:?}", bar);
    println!("{:?}", baz);

    // get 42 from foo
    if let RSEnum::Foo(value) = foo {
        println!("Value of foo is: {}", value);
    }
}
