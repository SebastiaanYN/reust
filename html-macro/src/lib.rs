use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use syn::parse_macro_input;

mod ast;
mod codegen;
mod parser;

use codegen::Codegen;

#[proc_macro_hack]
pub fn html(input: TokenStream) -> TokenStream {
    eprintln!("\n\n{:#?}\n\n", input);

    let element = parse_macro_input!(input as ast::Element);

    eprintln!("\n\n{:#?}\n\n", element);

    let expanded = element.gen();

    eprintln!("\n\n{}\n\n", expanded);

    TokenStream::from(expanded)
}
