use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, ItemFn, parse_macro_input};

/// Example 1: derive macro
#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl #name {
            pub fn hello() {
                println!("Hello from {}!", stringify!(#name));
            }
        }
    };

    expanded.into()
}

/// Example 2: attribute macro to wrap fn in tokio::spawn
#[proc_macro_attribute]
pub fn async_task(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let vis = &input_fn.vis;
    let name = &input_fn.sig.ident;
    let block = &input_fn.block;

    // This generates a new function with the same name that spawns
    // the body asynchronously, and returns immediately.
    let expanded = quote! {
        #vis fn #name() {
            tokio::spawn(async move {
                #block
            });
        }
    };

    expanded.into()
}
