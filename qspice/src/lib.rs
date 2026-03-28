extern crate proc_macro;

use proc_macro::TokenStream;

mod macros;
mod util;

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::main(args.into(), item.into()).into()
}
