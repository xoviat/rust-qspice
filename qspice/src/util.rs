use std::fmt::Display;

use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::parse::{Parse, ParseStream};
use syn::{
    AttrStyle, Attribute, FnArg, Signature, Token, Type, Visibility, braced, bracketed, token,
};

pub fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(error.into_compile_error());
    tokens
}

pub fn error<A: ToTokens, T: Display>(s: &mut TokenStream, obj: A, msg: T) {
    s.extend(syn::Error::new_spanned(obj.into_token_stream(), msg).into_compile_error())
}

/// Function signature and body.
///
/// Same as `syn`'s `ItemFn` except we keep the body as a TokenStream instead of
/// parsing it. This makes the macro not error if there's a syntax error in the body,
/// which helps IDE autocomplete work better.
#[derive(Debug, Clone)]
pub struct ItemFn {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
    pub brace_token: token::Brace,
    pub body: TokenStream,
}

impl Parse for ItemFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;

        let content;
        let brace_token = braced!(content in input);
        while content.peek(Token![#]) && content.peek2(Token![!]) {
            let content2;
            attrs.push(Attribute {
                pound_token: content.parse()?,
                style: AttrStyle::Inner(content.parse()?),
                bracket_token: bracketed!(content2 in content),
                meta: content2.parse()?,
            });
        }

        let mut body = Vec::new();
        while !content.is_empty() {
            body.push(content.parse::<TokenTree>()?);
        }
        let body = body.into_iter().collect();

        Ok(ItemFn {
            attrs,
            vis,
            sig,
            brace_token,
            body,
        })
    }
}

impl ToTokens for ItemFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            self.attrs
                .iter()
                .filter(|a| matches!(a.style, AttrStyle::Outer)),
        );
        self.vis.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            tokens.append_all(self.body.clone());
        });
    }
}

macro_rules! parse_type {
    ($($tt:tt)*) => {{
        let ty: syn::Type = syn::parse_quote! { $($tt)* };
        ty
    }}
}

pub(crate) enum ArgType {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    F32,
    F64,
    I64,
    U64,
}

impl TryInto<ArgType> for Type {
    type Error = ();

    fn try_into(self) -> Result<ArgType, Self::Error> {
        if self == parse_type! { bool } {
            Ok(ArgType::Bool)
        } else if self == parse_type! { i8 } {
            Ok(ArgType::I8)
        } else if self == parse_type! { u8 } {
            Ok(ArgType::U8)
        } else if self == parse_type! { i16 } {
            Ok(ArgType::I16)
        } else if self == parse_type! { u16 } {
            Ok(ArgType::U16)
        } else if self == parse_type! { i32 } {
            Ok(ArgType::I32)
        } else if self == parse_type! { u32 } {
            Ok(ArgType::U32)
        } else if self == parse_type! { f32 } {
            Ok(ArgType::F32)
        } else if self == parse_type! { f64 } {
            Ok(ArgType::F64)
        } else if self == parse_type! { i64 } {
            Ok(ArgType::I64)
        } else if self == parse_type! { u64 } {
            Ok(ArgType::U64)
        } else {
            Err(())
        }
    }
}

impl ArgType {
    pub fn ty(&self) -> TokenStream {
        match self {
            ArgType::Bool => quote! { bool },
            ArgType::I8 => quote! { i8 },
            ArgType::U8 => quote! { u8 },
            ArgType::I16 => quote! { i16 },
            ArgType::U16 => quote! { u16 },
            ArgType::I32 => quote! { i32 },
            ArgType::U32 => quote! { u32 },
            ArgType::F32 => quote! { f32 },
            ArgType::F64 => quote! { f64 },
            ArgType::I64 => quote! { i64 },
            ArgType::U64 => quote! { u64 },
        }
    }

    pub fn u(&self) -> TokenStream {
        match self {
            ArgType::Bool => quote! { b },
            ArgType::I8 => quote! { c },
            ArgType::U8 => quote! { uc },
            ArgType::I16 => quote! { s },
            ArgType::U16 => quote! { us },
            ArgType::I32 => quote! { i },
            ArgType::U32 => quote! { ui },
            ArgType::F32 => quote! { f },
            ArgType::F64 => quote! { d },
            ArgType::I64 => quote! { i64 },
            ArgType::U64 => quote! { ui64 },
        }
    }
}

pub(crate) enum ArgDir {
    Input,
    Output,
}

pub(crate) struct Argument {
    pub typ: ArgType,
    pub dir: ArgDir,
}

impl TryInto<Argument> for Type {
    type Error = ();

    fn try_into(self) -> Result<Argument, Self::Error> {
        match self {
            Type::Reference(r) => Ok(Argument {
                typ: (*r.elem).try_into()?,
                dir: ArgDir::Output,
            }),
            t => Ok(Argument {
                typ: t.try_into()?,
                dir: ArgDir::Input,
            }),
        }
    }
}

pub(crate) fn process_st_arg(arg_st: &FnArg) -> Result<TokenStream, &'static str> {
    let arg_st = if let FnArg::Typed(arg_st) = arg_st {
        *arg_st.ty.clone()
    } else {
        return Err("argument type is not correct");
    };

    let arg_st = if let Type::Reference(r) = arg_st {
        *r.elem
    } else {
        return Err("argument is not a reference");
    };

    Ok(arg_st.into_token_stream())
}

pub(crate) fn process_data_arg(arg_data: &FnArg) -> Result<Vec<Argument>, &'static str> {
    let arg_data = if let FnArg::Typed(arg_data) = arg_data {
        arg_data
    } else {
        return Err("argument type is not correct");
    };

    let arg_data = if let Type::Tuple(arg_data) = *arg_data.ty.clone() {
        arg_data
    } else {
        return Err("argument is not a tuple");
    };

    arg_data
        .elems
        .iter()
        .map(|t| t.clone().try_into())
        .collect::<Result<Vec<Argument>, _>>()
        .map_err(|_| "failed to parse tuple arguments")
}
