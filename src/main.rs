enum RSEnum {
    Foo(i32);
    Bar(String);
    Baz(Vec<String>);
}

fn main() {
    let foo = RSEnum::Foo(42);
    let bar = RSEnum::Bar("Hello".to_string());
    let baz = RSEnum::Baz(vec!["a".to_string(), "b".to_string()]);

    println!("{:?}", foo);
    println!("{:?}", bar);
    println!("{:?}", baz);
}
