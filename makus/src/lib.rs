mod enum_var_darling;
mod enum_var;

use crate::enum_var::process_enum_var;
use proc_macro::TokenStream;
use crate::enum_var_darling::process_enum_var_darling;

/// generate From Impls for enum's each variant
#[proc_macro_derive(EnumVar)]
pub fn enum_var(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_var(input).into()
}

#[proc_macro_derive(EnumVarDarling)]
pub fn enum_var_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_var_darling(input).into()
}
