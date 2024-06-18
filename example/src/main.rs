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
