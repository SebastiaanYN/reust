use proc_macro2::Delimiter;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, Error, Expr, Ident, LitStr, Token};

use crate::ast::*;

fn is_braced(input: ParseStream) -> bool {
    input.cursor().group(Delimiter::Brace).is_some()
}

fn parse_braced_expr(input: ParseStream) -> Result<Expr> {
    let content;
    braced!(content in input);
    content.parse::<Expr>()
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        // Opening element
        input.parse::<Token![<]>()?;
        let name = input.parse::<Ident>()?;

        // Events and attributes
        let mut attributes = Vec::new();
        let mut events = Vec::new();

        while !(input.peek(Token![/]) || input.peek(Token![>])) {
            if input.peek(Token![@]) {
                events.push(input.parse::<Event>()?);
            } else if input.peek(Ident) {
                attributes.push(input.parse::<Attribute>()?);
            } else {
                return Err(input.error("expected attribute or event"));
            }
        }

        // Self-closing element
        if let Ok(_) = input.parse::<Token![/]>() {
            input.parse::<Token![>]>()?;

            return Ok(Element {
                name,
                attributes,
                events,
                children: Vec::new(),
            });
        }

        input.parse::<Token![>]>()?;

        // Children
        let mut children = Vec::new();
        while !(input.peek(Token![<]) && input.peek2(Token![/])) {
            children.push(input.parse::<Child>()?);
        }

        // Closing element
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        input.parse::<Ident>().and_then(|closing_name| {
            if closing_name != name {
                Err(Error::new(
                    closing_name.span(),
                    format!("expected `{}`", name),
                ))
            } else {
                Ok(closing_name)
            }
        })?;
        input.parse::<Token![>]>()?;

        Ok(Element {
            name,
            attributes,
            events,
            children,
        })
    }
}

impl Parse for Child {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(child) = input.parse::<LitStr>() {
            Ok(Child::Text(child))
        } else if input.peek(Token![<]) {
            Ok(Child::Element(input.parse::<Element>()?))
        } else if is_braced(input) {
            Ok(Child::Expr(input.call(parse_braced_expr)?))
        } else {
            Err(input.error("expected string, element, or expression"))
        }
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;

        let value = if let Ok(value) = input.parse::<LitStr>() {
            AttributeValue::Str(value)
        } else if input.peek(Token![<]) {
            AttributeValue::Element(input.parse::<Element>()?)
        } else if is_braced(input) {
            AttributeValue::Expr(input.call(parse_braced_expr)?)
        } else {
            return Err(input.error("expected string, element, or expression"));
        };

        Ok(Attribute { name, value })
    }
}

impl Parse for Event {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![@]>()?;
        let name = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let expr = input.call(parse_braced_expr)?;

        Ok(Event { name, expr })
    }
}
