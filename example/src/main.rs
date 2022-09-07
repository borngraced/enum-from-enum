use derive_more::Display;
use enum_from_enum::EnumFromEnum;

// E.G, this converts from whatever Bar is to Foo::Bar(String) and
// whatever FooBar to Foo::FooBar(FooBar)
#[derive(Debug, EnumFromEnum)]
pub enum Foo {
    #[enum_from_enum("Bar")]
    Bar(String),
    #[enum_from_enum("FooBar")]
    FooBar(FooBar),
}

#[derive(Debug, Display)]
pub enum Bar {
    Foo(String),
}

#[derive(Debug, Display)]
pub enum FooBar {
    Foo(String),
}

fn foo_fn() -> Result<(), Foo> {
    Ok(bar_fn()?)
}

fn bar_fn() -> Result<(), Bar> {
    Err(Bar::Foo("Err".to_string()))
}

fn main() {
    println!("HELLO WORLD");
}
