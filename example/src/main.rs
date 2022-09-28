use derive_more::Display;
use enum_from_variant::EnumFromVariant;

// E.G, this converts from whatever Bar is to Foo::Bar(String) and
// whatever FooBar to Foo::FooBar(FooBar)
#[derive(Debug, EnumFromVariant, PartialEq, Eq)]
pub enum Foo {
    #[enum_from_variant("Bar")]
    Bar(String),
    #[enum_from_variant("FooBar")]
    FooBar(FooBar),
}

#[derive(Debug, Display, PartialEq, Eq)]
pub enum Bar {
    Foo(String),
}

#[derive(Debug, Display, PartialEq, Eq)]
pub enum FooBar {
    Foo(String),
}

fn foo_fn() -> Foo {
    bar_fn().into()
}

fn bar_fn() -> Bar {
    Bar::Foo("Hi babbbs".to_string())
}

#[test]
fn test_from_variant() {
    let bar = foo_fn();
    assert_eq!(Foo::Bar("Hi babbbs".to_string()), bar)
}

fn main() {}
