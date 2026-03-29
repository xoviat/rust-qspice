use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ReturnType, Type};

use crate::util::{
    ItemFn, error, process_data_arg, process_data_vars, process_st_arg, token_stream_with_error,
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
    let (st, args) = if fargs.len() == 4 {
        let (arg_st, arg_t, arg_data) = (&fargs[1], &fargs[2], &fargs[3]);

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

    let (vars, tup) = process_data_vars(&args);

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
            pub str: *mut ::core::ffi::c_char,
            pub bytes: *mut u8,
        }

        // TODO: validate args tuple as well
        /// main type validator
        #[inline]
        fn __qspice_main(_st: &mut #st) {}

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #f_name(opaque: *mut *mut u8, t: f64, data: *mut __u_data) {
            #[inline]
            fn fun(#fargs) {
                #f_body
            }

            unsafe {
                #vars

                if (*opaque).is_null() {
                    *opaque = Box::into_raw(Box::from(#st::default())) as *mut u8;
                }

                let mut qspice = ::qspice::QSpice::new();

                fun(&mut qspice, &mut *(*opaque as *mut #st), t, (#tup));
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn Destroy(opaque: *mut u8) {
            unsafe {
                let _ = Box::from_raw(opaque as *mut #st);
            }
        }

        #errors
    }
}

pub fn trunc(args: TokenStream, item: TokenStream) -> TokenStream {
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
    let (st, args) = if fargs.len() == 5 {
        let (arg_st, arg_t, arg_data, arg_tstep) = (&fargs[1], &fargs[2], &fargs[3], &fargs[4]);

        if let FnArg::Typed(arg_t) = arg_t
            && *arg_t.ty == parse_type! { f64 }
        {
            // argument valid
        } else {
            error(&mut errors, &arg_t, "argument type is not correct");
        };

        if let FnArg::Typed(arg_tstep) = arg_tstep
            && *arg_tstep.ty == parse_type! { &mut f64 }
        {
            // argument valid
        } else {
            error(&mut errors, &arg_tstep, "argument type is not correct");
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

    let (vars, tup) = process_data_vars(&args);

    quote! {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn Trunc(opaque: *mut u8, t: f64, data: *mut __u_data, timestep: *mut f64) {
            #[inline]
            fn fun(#fargs) {
                #f_body
            }

            unsafe {
                #vars

                let mut qspice = ::qspice::QSpice::new();

                __qspice_main(&mut *(opaque as *mut #st));
                fun(&mut qspice, &mut *(opaque as *mut #st), t, (#tup), &mut *timestep);
            }
        }

        #errors
    }
}

pub fn max(args: TokenStream, item: TokenStream) -> TokenStream {
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

    if let ReturnType::Type(_, ty) = &f.sig.output
        && **ty == parse_type! { f64 }
    {
        // return type valid
    } else {
        error(&mut errors, &f.sig, "function has incorrect return type");
    }

    let fargs = f.sig.inputs.clone();
    let st = if fargs.len() == 3 {
        let (arg_st, arg_t) = (&fargs[1], &fargs[2]);

        if let FnArg::Typed(arg_t) = arg_t
            && *arg_t.ty == parse_type! { f64 }
        {
            // argument valid
        } else {
            error(&mut errors, &arg_t, "argument type is not correct");
        };

        process_st_arg(arg_st)
            .map_err(|e| error(&mut errors, &arg_st, e))
            .unwrap_or(TokenStream::new())
    } else {
        error(&mut errors, &args, "incorrect number of arguments");

        TokenStream::new()
    };

    let f_body = f.body;

    quote! {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn MaxExtStepSize(opaque: *mut u8, t: f64) -> f64 {
            #[inline]
            fn fun(#fargs) -> f64 {
                #f_body
            }

            unsafe {
                let mut qspice = ::qspice::QSpice::new();

                __qspice_main(&mut *(opaque as *mut #st));
                fun(&mut qspice, &mut *(opaque as *mut #st), t)
            }
        }

        #errors
    }
}

#[test]
fn test_main() {
    main(
        quote! {},
        quote! {
            fn cont(
                cont: &mut sCONT,
                t: f64,
                data: (
                    f32,
                    f32,
                    f32,
                    f32,
                    f32,
                    &mut bool,
                    &mut f32,
                    &mut f32,
                    &mut f32,
                    &mut f32,
                    &mut f32,
                    &mut f32,
                    &mut f32,
                    &mut f32,
                    &mut f32,
                ),
            ) {
            }
        },
    );
}
