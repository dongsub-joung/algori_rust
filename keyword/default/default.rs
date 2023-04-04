fn main() {
    let a: usize= Default::default();
    print!("{}", a);

    let b: Foo= Foo::default();
    print!("{:?}", b);
}

#[derive(Debug)]
struct Foo{
    x: f32,
}

impl Default for Foo{
    fn default() -> Self{
        Foo{x: 3.14}
    }
}
