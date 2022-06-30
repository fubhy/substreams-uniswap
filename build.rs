use anyhow::{Ok, Result};
// use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    // TODO: Abigen should use a `Log` trait rather than a direct dependence on the protobufs Log

    // Abigen::new("UniswapV2Factory", "abi/factory.json")?
    //     .generate()?
    //     .write_to_file("src/abi/factory.rs")?;

    // Abigen::new("UniswapV2Pair", "abi/pair.json")?
    //     .generate()?
    //     .write_to_file("src/abi/pair.rs")?;

    Ok(())
}
