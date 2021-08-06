use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse_macro_input;

mod parser;

#[proc_macro_hack]
pub fn html(input: TokenStream) -> TokenStream {
    eprintln!("\n\n{:#?}\n\n", input);

    let element = parse_macro_input!(input as parser::Element);

    eprintln!("\n\n{:#?}\n\n", element);

    TokenStream::from(quote! {})
}
