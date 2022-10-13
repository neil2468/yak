use anyhow::Result;
use yak_core::NodeAddress;

#[test]
fn main() -> Result<()> {
    let addr = NodeAddress::try_from("node# 1")?;
    println!("addr = {}", addr);
    Ok(())
}
