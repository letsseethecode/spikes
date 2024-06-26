use proc_macro::TokenStream;
use quote::quote;
use std::fmt::Display;
use syn::{parse::Parse, parse_macro_input, Ident, Token};

pub(crate) mod keyword {
    syn::custom_keyword!(go);
    syn::custom_keyword!(take);
    syn::custom_keyword!(drop);
    syn::custom_keyword!(give);
    syn::custom_keyword!(to);
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::North => write!(f, "north"),
            Self::East => write!(f, "east"),
            Self::South => write!(f, "south"),
            Self::West => write!(f, "west"),
        }
    }
}

impl Parse for Direction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut result: Option<Direction> = None;
        if input.peek(Ident) {
            let direction = input.parse::<Ident>()?;
            result = match direction.to_string().as_str() {
                "north" => Some(Direction::North),
                "east" => Some(Direction::East),
                "south" => Some(Direction::South),
                "west" => Some(Direction::West),
                _ => None,
            };
        }
        result.ok_or(syn::Error::new(
            input.span(),
            "expected: north, east, south or west",
        ))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Story {
    pub messages: Vec<String>,
    pub items: Vec<String>,
}

impl Parse for Story {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut result = Story {
            messages: vec![],
            items: vec![],
        };
        loop {
            let la = input.lookahead1();
            if la.peek(keyword::give) {
                input.parse::<keyword::give>()?;
                let item = input.parse::<Ident>()?;
                input.parse::<keyword::to>()?;
                let person = input.parse::<Ident>()?;
                if !result.items.contains(&item.to_string()) {
                    return Err(syn::Error::new(
                        item.span(),
                        format!(
                            "Impossible, you were not carrying any {} at this point.",
                            item
                        ),
                    ));
                }
                result.items.retain(|s| s != &item.to_string());
                result
                    .messages
                    .push(format!("You gave {} to {}.", item, person));
            } else if la.peek(keyword::take) {
                input.parse::<keyword::take>()?;
                let item = input.parse::<Ident>()?;
                result.items.push(item.to_string());
                result.messages.push(format!("You took a {}.", item));
            } else if la.peek(keyword::drop) {
                input.parse::<keyword::drop>()?;
                let item = input.parse::<Ident>()?;
                if !result.items.contains(&item.to_string()) {
                    return Err(syn::Error::new(
                        item.span(),
                        format!(
                            "Impossible, you were not carrying any {} at this point.",
                            item
                        ),
                    ));
                }
                result.items.retain(|s| s != &item.to_string());
                result.messages.push(format!("You dropped a {}.", item));
            } else if la.peek(keyword::go) {
                input.parse::<keyword::go>()?;
                let dir = input.parse::<Direction>()?;
                result.messages.push(format!("You headed {}.", dir));
            } else {
                break;
            }
            input.parse::<Token![.]>()?;
        }
        Ok(result)
    }
}

#[proc_macro]
pub fn story(input: TokenStream) -> TokenStream {
    let result: Story = parse_macro_input!(input);
    let steps = result.messages.iter().fold(quote! {}, |acc, f| {
        let dir = f;
        quote! {
            #acc
            println!("{}", #dir);
        }
    });
    quote! {
        println!("This is your story!");
        #steps
        println!("The END!");
    }
    .into()
}
