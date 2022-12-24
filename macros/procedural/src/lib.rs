use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn pass_through(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Enumerator)]
pub fn derive_enumerator(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    
    let variants = if let syn::Data::Enum(ref enum_data) = ast.data {
        let syn::DataEnum { variants, .. } = enum_data;

        variants.iter().map(|variant| {
            if variant.fields != syn::Fields::Unit {
                panic!("Cannot derive enumerator for enum with non-unit variants!");
            }

            variant.ident.clone()
        })
    } else {
        panic!("Cannot derive enumerator for structs!");
    };

    let expanded = quote! {
        impl #name {
            fn enumerator() -> impl Iterator<Item = #name> {
                [#(#name::#variants),*].into_iter()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Datapoint)]
pub fn derive_datapoint(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl Datapoint for #name {
            fn record(&self) -> String {
                format!("{:#?}", self) 
            }
        }
    };

    TokenStream::from(expanded)
}   

