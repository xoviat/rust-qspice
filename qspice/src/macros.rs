
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ReturnType, Type};

use crate::util::{
    ArgDir, ItemFn, error, process_data_arg, process_st_arg, token_stream_with_error,
};

macro_rules! parse_type {
    ($($tt:tt)*) => {{
        let ty: syn::Type = syn::parse_quote! { $($tt)* };
        ty
    }}
}

pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut errors = TokenStream::new();

    // If any of the steps for this macro fail, we still want to expand to an item that is as close
    // to the expected output as possible. This helps out IDEs such that completions and other
    // related features keep working.
    let f: ItemFn = match syn::parse2(item.clone()) {
        Ok(x) => x,
        Err(e) => return token_stream_with_error(item, e),
    };

    // This function has no arguments
    let returns_impl_trait = match &f.sig.output {
        ReturnType::Type(_, ty) => matches!(**ty, Type::ImplTrait(_)),
        _ => false,
    };

    if f.sig.asyncness.is_some() || returns_impl_trait {
        error(
            &mut errors,
            &f.sig,
            "function must not be async or impl trait",
        );
    }
    if !f.sig.generics.params.is_empty() {
        error(&mut errors, &f.sig, "function must not be generic");
    }
    if !f.sig.generics.where_clause.is_none() {
        error(
            &mut errors,
            &f.sig,
            "function must not have `where` clauses",
        );
    }
    if !f.sig.abi.is_none() {
        error(
            &mut errors,
            &f.sig,
            "function must not have an ABI qualifier",
        );
    }
    if !f.sig.variadic.is_none() {
        error(&mut errors, &f.sig, "function must not be variadic");
    }

    let fargs = f.sig.inputs.clone();
    let (st, args) = if fargs.len() == 3 {
        let (arg_st, arg_t, arg_data) = (&fargs[0], &fargs[1], &fargs[2]);

        if let FnArg::Typed(arg_t) = arg_t
            && *arg_t.ty == parse_type! { f64 }
        {
            // argument valid
        } else {
            error(&mut errors, &arg_t, "argument type is not correct");
        };

        (
            process_st_arg(arg_st)
                .map_err(|e| error(&mut errors, &arg_st, e))
                .unwrap_or(TokenStream::new()),
            process_data_arg(arg_data)
                .map_err(|e| error(&mut errors, &arg_data, e))
                .unwrap_or(Vec::new()),
        )
    } else {
        error(&mut errors, &args, "incorrect number of arguments");

        (TokenStream::new(), Vec::new())
    };

    let f_body = f.body;
    let f_name = f.sig.ident;

    let vars: TokenStream = args
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            let pfx = match arg.dir {
                ArgDir::Input => TokenStream::new(),
                ArgDir::Output => {
                    quote! { &mut }
                }
            };

            let dec = format_ident!("_{}", i);
            let off = format_ident!("{}", i);
            let u = arg.typ.u();
            let ty = arg.typ.ty();
            quote! { let #dec: #pfx #ty = #pfx (*data.add(#off)).#u; }
        })
        .collect();

    let tup: TokenStream = (0..args.len())
        .map(|i| {
            let dec = format_ident!("_{}", i);
            quote! { #dec, }
        })
        .collect();

    quote! {
        #[repr(C)]
        union __u_data {
            pub b: bool,
            pub c: i8,
            pub uc: u8,
            pub s: i16,
            pub us: u16,
            pub i: i32,
            pub ui: u32,
            pub f: f32,
            pub d: f64,
            pub i64: i64,
            pub ui64: u64,
            pub str: *mut c_char,
            pub bytes: *mut u8,
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #f_name(opaque: *mut *mut u8, t: f64, data: *mut __u_data) {
            fn fun(#fargs) {
                #f_body
            }

            unsafe {
                #vars

                if (*opaque).is_null() {
                    *opaque = Box::into_raw(Box::from(#st::default())) as *mut u8;
                }

                fun(&mut *(*opaque as *mut #st), t, (#tup));
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn Destroy(opaque: *mut u8) {
            unsafe {
                let _ = Box::from_raw(opaque as *mut #st);
            }
        }
    }
}
