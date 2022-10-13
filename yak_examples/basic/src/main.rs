use anyhow::Result;
use yak_core::Address;

fn main() -> Result<()> {
    let addr = Address::try_from("node#1")?;
    println!("addr = {}", addr);
    Ok(())
}
