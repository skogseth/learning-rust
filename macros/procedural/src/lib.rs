use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn pass_through(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Enumerator)]
pub fn derive(input: TokenStream) -> TokenStream {
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

    expanded.into()
}

