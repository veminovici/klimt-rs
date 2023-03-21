use proc_macro::TokenStream;

mod plugin;

/// Generates a dynamic plugin entry point function for the given `Plugin` type.
#[proc_macro_derive(DynamicPlugin)]
pub fn derive_dynamic_plugin(input: TokenStream) -> TokenStream {
    plugin::derive_dynamic_plugin(input)
}
