use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum NodeAddressError {
    #[error("invalid value for an address")]
    Invalid,
}

/// Valid addresses are unicode strings that...
/// * contain at least one character
/// * contain no control or whitespace characters
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeAddress {
    val: String,
}

/// Try to convert a `&str` to an `Address`.
///
/// Implementing `TryFrom` auto implements `TryInto`.
///
/// # Provides
/// ```
/// use yak_core::NodeAddress;
/// let addr = NodeAddress::try_from("p_123").unwrap();
/// let addr: NodeAddress = "p_123".try_into().unwrap();
/// ```
impl TryFrom<&str> for NodeAddress {
    type Error = NodeAddressError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if !s.is_empty() && s.chars().all(|c| !c.is_control() && !c.is_whitespace()) {
            return Ok(NodeAddress {
                val: String::from(s),
            });
        }
        Err(NodeAddressError::Invalid)
    }
}

/// Implementing `Display` auto implements `ToString`.
///
/// Provides...
/// * `println!("{}", addr)`
/// * `addr.to_string()`
impl std::fmt::Display for NodeAddress {
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
            assert!(NodeAddress::try_from(v).is_ok());
            assert!(<&str as TryInto<NodeAddress>>::try_into(v).is_ok());
        }
    }

    #[test]
    fn create_err() {
        let vals = vec!["node _123", " node_123", "node_123 ", "node\t", "node\n"];
        for v in vals {
            assert_eq!(NodeAddress::try_from(v), Err(NodeAddressError::Invalid));
            assert_eq!(
                <&str as TryInto<NodeAddress>>::try_into(v),
                Err(NodeAddressError::Invalid)
            );
        }
    }
}
