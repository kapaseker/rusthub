use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_var(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    // get generics from input
    let generics = input.generics;

    let variants = match input.data {
        syn::Data::Enum(e) => e.variants,
        _ => panic!("EnumVar can only be applied to enums"),
    };

    // get ident and fields from variants
    let qt = variants.into_iter().map(|v| {
        let variant_ident = v.ident;
        let fields = v.fields;
        match fields {
            syn::Fields::Unnamed(field) => {
                if field.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let f = field.unnamed.first().unwrap();
                    let ty = &f.ty;
                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(value: #ty) -> Self {
                                Self::#variant_ident(value)
                            }
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });

    quote! {
        #(#qt)*
    }
}
