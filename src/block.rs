use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use unicode_xid::UnicodeXID;

// Define custom keywords
mod kw {
    syn::custom_keyword!(describe);
    syn::custom_keyword!(context);
    syn::custom_keyword!(it);
    syn::custom_keyword!(test);
    syn::custom_keyword!(before);
    syn::custom_keyword!(after);
}

pub struct Root(pub(crate) Describe);

impl Parse for Root {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut before = vec![];
        let mut after = vec![];
        let mut blocks = vec![];

        while !input.is_empty() {
            let block = input.parse::<DescribeBlock>()?;
            match block {
                DescribeBlock::Regular(block) => blocks.push(block),
                DescribeBlock::Before(block) => before.push(block),
                DescribeBlock::After(block) => after.push(block),
            }
        }

        Ok(Root(Describe {
            name: syn::Ident::new("speculate", Span::call_site()),
            before,
            after,
            blocks,
        }))
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Block {
    Describe(Describe),
    It(It),
    Item(syn::Item),
}

impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        // Try to parse as Describe
        if lookahead.peek(kw::describe) || lookahead.peek(kw::context) {
            return Ok(Block::Describe(input.parse()?));
        }

        // Try to parse as It (which can start with attributes or it/test keywords)
        if lookahead.peek(syn::Token![#]) || lookahead.peek(kw::it) || lookahead.peek(kw::test) {
            return Ok(Block::It(input.parse()?));
        }

        // Otherwise parse as Item
        Ok(Block::Item(input.parse()?))
    }
}

#[allow(clippy::large_enum_variant)]
enum DescribeBlock {
    Regular(Block),
    Before(syn::Block),
    After(syn::Block),
}

impl Parse for DescribeBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::before) {
            input.parse::<kw::before>()?;
            let block = input.parse::<syn::Block>()?;
            return Ok(DescribeBlock::Before(block));
        }

        if lookahead.peek(kw::after) {
            input.parse::<kw::after>()?;
            let block = input.parse::<syn::Block>()?;
            return Ok(DescribeBlock::After(block));
        }

        Ok(DescribeBlock::Regular(input.parse()?))
    }
}

#[derive(Clone)]
pub struct Describe {
    pub name: syn::Ident,
    pub before: Vec<syn::Block>,
    pub after: Vec<syn::Block>,
    pub blocks: Vec<Block>,
}

impl Parse for Describe {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse 'describe' or 'context'
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::describe) {
            input.parse::<kw::describe>()?;
        } else if lookahead.peek(kw::context) {
            input.parse::<kw::context>()?;
        } else {
            return Err(lookahead.error());
        }

        // Parse the name
        let name_lit = input.parse::<syn::LitStr>()?;

        // Parse the braced content
        let content;
        syn::braced!(content in input);
        let root = content.parse::<Root>()?;

        let mut describe = root.0;
        describe.name = litstr_to_ident(&name_lit);

        Ok(describe)
    }
}

#[derive(Clone)]
pub struct It {
    pub name: syn::Ident,
    pub attributes: Vec<syn::Attribute>,
    pub block: syn::Block,
}

impl Parse for It {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse attributes
        let attrs = input.call(syn::Attribute::parse_outer)?;

        // Parse 'it' or 'test'
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::it) {
            input.parse::<kw::it>()?;
        } else if lookahead.peek(kw::test) {
            input.parse::<kw::test>()?;
        } else {
            return Err(lookahead.error());
        }

        // Parse the name as an identifier
        let name = input.parse::<syn::Ident>()?;

        // Parse the block
        let block = input.parse::<syn::Block>()?;

        Ok(It {
            name,
            attributes: attrs,
            block,
        })
    }
}

fn litstr_to_ident(l: &syn::LitStr) -> syn::Ident {
    let string = l.value();
    let mut id = String::with_capacity(string.len());

    if string.is_empty() {
        return syn::Ident::new("_", l.span());
    }

    let mut chars = string.chars();
    let mut added_underscore = false;

    let first_ch = chars.next().unwrap();

    if !UnicodeXID::is_xid_start(first_ch) {
        id.push('_');

        if UnicodeXID::is_xid_continue(first_ch) {
            id.push(first_ch);
        } else {
            added_underscore = true;
        }
    } else {
        id.push(first_ch);
    }

    for ch in chars {
        if UnicodeXID::is_xid_continue(ch) {
            id.push(ch);
            added_underscore = false;
        } else if !added_underscore {
            id.push('_');
            added_underscore = true;
        }
    }

    if id.as_bytes()[id.len() - 1] == b'_' {
        id.pop();
    }

    syn::Ident::new(&id, l.span())
}
