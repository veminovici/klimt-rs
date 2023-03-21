use klimt_plugin::Plugin;

#[derive(klimt_derive::DynamicPlugin)]
pub struct MyLogging {}

impl Plugin for MyLogging {
    fn print_name(&self) {
        println!("My logging is here");
    }
}
