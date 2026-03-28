extern crate proc_macro;

use proc_macro::TokenStream;

mod macros;
mod util;

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::main(args.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn max(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::max(args.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn trunc(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::trunc(args.into(), item.into()).into()
}
