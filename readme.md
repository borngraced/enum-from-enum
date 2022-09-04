### `EnumFromEnum` is very useful for generating `From<T>` trait from one enum to another enum.

Currently, this crate can only convert enum variant with only inner type such as `String` and `Enum`
type just like the example below. Can not be used for tuple, struct etc for now .

## USAGE:

```rust
use enum_from_enum::EnumFromEnum;
use derive_more::Display;
// E.G, this converts from whatever Bar is to Foo::Bar(String) and.
// whatever FooBar to Foo::FooBar(FooBar).
#[derive(Debug, EnumFromEnum)]
pub enum Foo {
    #[enum_from_enum("Bar")]
    Bar(String),
    #[enum_from_enum("FooBar")]
    FooBar(FooBar),
}
///
#[derive(Debug, Display)]
pub enum Bar {
    Foo(String),
}
///
#[derive(Debug, Display)]
pub enum FooBar {
    Foo(String),
}
///
fn foo_fn() -> Result<(), Foo> {
    Ok(bar_fn()?)
}
///
fn bar_fn() -> Result<(), Bar> {
    Err(Bar::Foo("Err".to_string()))
}
```
