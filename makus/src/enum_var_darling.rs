use darling::ast::Data;
use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::empty;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct DataFromDerive {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<DataFromVariant, ()>,
}

#[derive(Debug, FromVariant)]
struct DataFromVariant {
    ident: syn::Ident,
    fields: darling::ast::Fields<DataField>,
}

#[derive(Debug, FromField)]
struct DataField {
    ty: syn::Type,
}

pub(crate) fn process_enum_var_darling(input: DeriveInput) -> TokenStream {
    if let DataFromDerive {
        ident,
        generics,
        data: Data::Enum(data),
    } = DataFromDerive::from_derive_input(&input).unwrap()
    {
        let qt = data.into_iter().map(|v| {
            let variant_ident = v.ident;
            let field = v.fields;
            if field.fields.len() != 1 {
                quote! {}
            } else {
                let f = field.fields.first().unwrap();
                let ty = &f.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(value: #ty) -> Self {
                            Self::#variant_ident(value)
                        }
                    }
                }
            }
        });

        quote! {
            #(#qt)*
        }
    } else {
        quote! {}
    }
}
