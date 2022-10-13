//! Core types.

// TODO #![warn(missing_docs)]

mod address;

pub use address::*;

#[cfg(test)]
mod tests {
    use crate::Address;

    #[test]
    fn one() {
        let addr = Address::try_from("node#12").unwrap();
        println!("addr = {}", addr);
    }
}
