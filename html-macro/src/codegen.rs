use proc_macro2::TokenStream;
use quote::quote;

use crate::ast::*;

pub trait Codegen {
    fn gen(&self) -> TokenStream;
}

impl Codegen for Element {
    fn gen(&self) -> TokenStream {
        let name = format!("{}", self.name);
        let props = self.attributes.iter().map(Attribute::gen);
        let children = self.children.iter().map(Child::gen);
        let events = self.events.iter().map(Event::gen);

        quote! {
            {
                let el = element(
                    #name,
                    &[
                        #(#props),*
                    ],
                );

                #(el.append_children(#children.into_nodes());)*

                #(#events)*

                el
            }
        }
    }
}

impl Codegen for Child {
    fn gen(&self) -> TokenStream {
        match self {
            Self::Text(txt) => quote! { text(#txt) },
            Self::Element(el) => el.gen(),
            Self::Expr(expr) => quote! {
                {
                    let el = #expr;

                    // {
                    //     let el = el.clone();

                    //     #expr.subscribe(move |value| el.set_text_content(value));
                    // }

                    el
                }
            },
        }
    }
}

impl Codegen for Attribute {
    fn gen(&self) -> TokenStream {
        let name = format!("{}", self.name);
        let value = self.value.gen();

        quote! {
            (#name, #value)
        }
    }
}

impl Codegen for AttributeValue {
    fn gen(&self) -> TokenStream {
        match self {
            Self::Str(s) => quote! { #s },
            Self::Element(el) => el.gen(),
            Self::Expr(expr) => quote! { #expr },
        }
    }
}

impl Codegen for Event {
    fn gen(&self) -> TokenStream {
        let name = format!("{}", self.name);
        let expr = &self.expr;

        quote! {
            {
                el.add_event_listener(#name, #expr);
            }
        }
    }
}
