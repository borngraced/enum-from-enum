`enum-from-variant` crate provides the EnumFromVariant macro, which simplifies the generation of the From<T> trait for converting one enum variant to another enum variant. This is particularly useful when you need to handle error conversions or map different enum types in your Rust code.

### USAGE:

Consider the following example where we convert between different enum types using the EnumFromVariant macro:

```rust
use enum_from_variant::EnumFromVariant;
use derive_more::Display;

#[derive(Debug, EnumFromVariant)]
pub enum MainError {
    #[enum_from_variant("NetworkError")]
    Network(String),
    #[enum_from_variant("DatabaseError")]
    Database(DatabaseError),
}

#[derive(Debug, Display)]
pub enum NetworkError {
    Timeout(String),
}

#[derive(Debug, Display)]
pub enum DatabaseError {
    ConnectionFailed(String),
}

fn network_request() -> Result<(), MainError> {
    Err(NetworkError::Timeout("Network timeout".to_string()).into())
}

fn main() {
    match network_request() {
        Ok(_) => println!("Request succeeded"),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

### Limitations
`Current Support`: The macro only supports enum variants with basic inner types like String and other enums.
`Unsupported Types`: Tuple variants, struct variants, and more complex inner types are not supported at this time.
