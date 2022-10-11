use crate::error::Error;

/// A generic address type.
///
/// Addresses can be created from strings with the form
/// `<one_or_more_ascii_alphabetic>_<one_or_more_ascii_digit>`.
///
/// Alphabetic part is __stored as lower case__.
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Address {
    prefix: String,
    id: u32,
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
        f.write_str(&String::from(self))
    }
}

/// Convert an `&Address` to a `String`.
///
/// Implementing `From` auto implements `Into`.
///
/// # Provides
/// * `String::from(&addr)`
/// * `let s: String = addr.into()`
///
/// # Examples
/// ```
/// use yak_core::{Address, Error};
/// # fn main() -> Result<(), Error> {
/// let addr = Address::try_from("a_123")?;
/// assert_eq!(String::from(&addr), "a_123");
/// # Ok(())
/// # }
/// ```
impl From<&Address> for String {
    fn from(a: &Address) -> Self {
        format!("{}_{}", a.prefix, a.id)
    }
}

/// Try to convert a `&str` to an `Address`.
///
/// Implementing `TryFrom` auto implements `TryInto`.
///
/// # Provides
/// * `let addr = Address::try_from("p_123")?;`
/// * `let addr: Address = "p_123".try_into()?;`
///
/// # Examples
/// ```
/// use yak_core::Address;
///
/// match Address::try_from("p_123") {
///     Ok(addr) => println!("Created address: {}", addr),
///     Err(e) => println!("Error: {}", e)
/// }
/// ```
/// ```
/// use yak_core::{Address, Error};
///
/// fn main() -> Result<(), Error> {
///     let addr: Address = "p_123".try_into()?;
///     println!("Created address {}", addr);
///     Ok(())
/// }
/// ```
impl TryFrom<&str> for Address {
    type Error = crate::error::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let tmp: Vec<&str> = s.split('_').collect();
        if tmp.len() == 2
            && tmp[0].len() > 0
            && tmp[1].len() > 0
            && tmp[0].chars().all(|x| x.is_ascii_alphabetic())
            && tmp[1].chars().all(|x| x.is_ascii_digit())
        {
            if let Ok(id) = tmp[1].parse::<u32>() {
                let prefix = String::from(tmp[0]).to_lowercase();
                return Ok(Address { prefix, id });
            }
        }
        Err(Error::InvalidAddress)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::InvalidAddress;

    #[test]
    fn create_address_ok() {
        assert!(Address::try_from("neil_957").is_ok());
        assert_eq!(
            Address::try_from("neilXXX_957").unwrap(),
            Address {
                prefix: String::from("neilxxx"),
                id: 957
            }
        );
        assert!(Address::try_from("p_1").is_ok());
    }

    #[test]
    fn create_address_err() {
        assert_eq!(Address::try_from(""), Err(InvalidAddress));
        assert_eq!(Address::try_from("  "), Err(InvalidAddress));
        assert_eq!(Address::try_from(" p_223"), Err(InvalidAddress));
        assert_eq!(Address::try_from("p_223 "), Err(InvalidAddress));
        assert_eq!(Address::try_from("3_223"), Err(InvalidAddress));
        assert_eq!(Address::try_from("3_p"), Err(InvalidAddress));
        assert_eq!(Address::try_from("p223"), Err(InvalidAddress));
        assert_eq!(Address::try_from("_223"), Err(InvalidAddress));
        assert_eq!(Address::try_from("__223"), Err(InvalidAddress));
        assert_eq!(Address::try_from("p__223"), Err(InvalidAddress));
        assert_eq!(Address::try_from("p_"), Err(InvalidAddress));
        assert_eq!(Address::try_from("p_ "), Err(InvalidAddress));
        assert_eq!(Address::try_from("p_2_23"), Err(InvalidAddress));
    }
}
