use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum AddressError {
    #[error("invalid value for an address")]
    Invalid,
}

/// Valid addresses are unicode strings that...
/// * contain at least one character
/// * contain no control or whitespace characters
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address {
    val: String,
}

impl Address {
    fn parse<S: Into<String>>(s: S) -> Result<Self, AddressError> {
        let s: String = s.into();
        if !s.is_empty() && s.chars().all(|c| !c.is_control() && !c.is_whitespace()) {
            return Ok(Address { val: s });
        }
        Err(AddressError::Invalid)
    }
}

/// Try to convert a `&str` to an `Address`.
///
/// # Examples
/// ```
/// use yak_core::Address;
/// let addr = Address::try_from("p_123").unwrap();
/// let addr: Address = "p_123".try_into().unwrap();
/// ```
impl TryFrom<&str> for Address {
    type Error = AddressError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse(s)
    }
}

/// Try to convert a `&String` to an `Address`.
///
/// # Examples
/// ```
/// use yak_core::Address;
/// let s = String::from("123");
/// let addr = Address::try_from(&s).unwrap();
/// let addr: Address = (&s).try_into().unwrap();
/// ```
impl TryFrom<&String> for Address {
    type Error = AddressError;
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Self::parse(s)
    }
}

/// Try to convert a `String` to an `Address`.
///
/// # Examples
/// ```
/// use yak_core::Address;
/// let s = String::from("123");
/// let addr = Address::try_from(s);
/// let s = String::from("123");
/// let addr:Address = s.try_into().unwrap();
/// ```
impl TryFrom<String> for Address {
    type Error = AddressError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::parse(s)
    }
}

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
        }
    }

    #[test]
    fn create_err() {
        let vals = vec!["node _123", " node_123", "node_123 ", "node\t", "node\n"];
        for v in vals {
            assert_eq!(Address::try_from(v), Err(AddressError::Invalid));
        }
    }
}
