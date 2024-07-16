use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn trust_me(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let block_stmts = &input_fn.block.stmts;
    input_fn.block.stmts = vec![syn::parse_quote! {
        unsafe { #(#block_stmts)* }
    }];
    TokenStream::from(quote!(#input_fn))
}
