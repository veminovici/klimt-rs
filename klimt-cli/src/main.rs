use anyhow::{anyhow, Result};
use klimt_plugin::dynamically_load_plugin;

fn main() -> Result<()> {
    println!("Starting");

    // lets try to load a library
    let res = unsafe { dynamically_load_plugin("./target/debug/libklimt_log.dylib") };
    let (_library, plugin) = res.map_err(|e| anyhow!(e))?;

    println!("Trying to print");
    plugin.print_name();

    println!("Done - all good!");
    Ok(())
}
