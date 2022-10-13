use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum AddressError {
    #[error("invalid value for an address")]
    Invalid,
}

/// Valid addresses are unicode strings that...
/// * contain at least one character
/// * contain no control or whitespace characters
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Address {
    val: String,
}

/// Try to convert a `&str` to an `Address`.
///
/// Implementing `TryFrom` auto implements `TryInto`.
///
/// # Provides
/// * `let addr = Address::try_from("p_123")?;`
/// * `let addr: Address = "p_123".try_into()?;`
///
impl TryFrom<&str> for Address {
    type Error = AddressError;
    fn try_from(s: &str) -> Result<Self, AddressError> {
        if !s.is_empty() && s.chars().all(|c| !c.is_control() && !c.is_whitespace()) {
            return Ok(Address {
                val: String::from(s),
            });
        }
        Err(AddressError::Invalid)
    }
}

/// Display an `Address`.
///
/// Implementing `Display` auto implements `ToString`.
///
/// Provides...
/// * `println!("{}", addr)`
/// * `addr.to_string()`
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ok() {
        let vals = vec![
            "node",
            "node_123",
            "123_node",
            "node#123",
            "Node123",
            "ANodeSomewhere",
            "Happy😀Node",
        ];
        for v in vals {
            assert!(Address::try_from(v).is_ok());
            assert!(<&str as TryInto<Address>>::try_into(v).is_ok());
        }
    }

    #[test]
    fn create_err() {
        let vals = vec!["node _123", " node_123", "node_123 ", "node\t", "node\n"];
        for v in vals {
            assert_eq!(Address::try_from(v), Err(AddressError::Invalid));
            assert_eq!(
                <&str as TryInto<Address>>::try_into(v),
                Err(AddressError::Invalid)
            );
        }
    }
}
