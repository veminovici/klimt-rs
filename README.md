# KLIMT
A set of crates to test how we can create a dynamic library and then load it into an application.

## KLIMT-PLUGIN Crate
A create which defines the *Plugin* trait and the function that can load a dynamic library which exposes a *_kaantor_create_plugin* function.

## KLIMT-DERIVE Crate
A *proc-macro* create which implements the macro which can be mark a structure as an exportable *plugin*.

## KLIMT-LOG Crate
A *cdylib* crate which exports a *plugin* implemented by *MyLogging* structure.

```rust
#[derive(klimt_derive::DynamicPlugin)]
pub struct MyLogging {}

impl Plugin for MyLogging {
    fn print_name(&self) {
        println!("My logging is here");
    }
}
```

## KLIMG-CLI Application
A CLI application which loads the dynamic library and then calls the *plugin*.

```rust
let res = unsafe { dynamically_load_plugin("./target/debug/libklimt_log.dylib") };
let (_library, plugin) = res.map_err(|e| anyhow!(e))?;
plugin.print_name();
```