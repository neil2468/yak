#![warn(missing_docs)]

//! Core types.

mod address;
mod error;

pub use address::*;
pub use error::*;

#[cfg(test)]
mod tests {
    use crate::{address::Address, error::Error};

    #[test]
    fn is_pub_address() -> Result<(), Error> {
        Address::try_from("neil_999")?;
        Ok(())
    }
}
